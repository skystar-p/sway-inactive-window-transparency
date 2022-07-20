use anyhow::bail;
use tokio_i3ipc::{
    event::{Event, Subscribe, WindowChange},
    reply::{Node, Workspace},
    I3,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opacity = std::env::vars()
        .find(|(k, _)| k == "INACTIVE_OPACITY")
        .map(|(_, v)| v)
        .unwrap_or("0.8".into());

    let mut conn = I3::connect().await?;

    // find focused workspace
    let focused_workspace = match find_focused_workspace(&mut conn).await? {
        Some(w) => w,
        None => bail!("no focused workspace"),
    };
    tracing::debug!("initial focused workspace: {:?}", focused_workspace);
    let mut prev_focused_workspace_num = focused_workspace.num;

    // find focused node
    let mut prev_focused_node = match find_focused_node(&mut conn).await? {
        Some(node) => node,
        None => {
            bail!("Could not find focused node in initial tree");
        }
    };
    tracing::debug!("initial focused node: {:?}", prev_focused_node);

    // subscribe to window events
    conn.subscribe([Subscribe::Window]).await?;

    let mut listener = conn.listen();
    let mut conn = I3::connect().await?;
    while let Some(event) = listener.next().await {
        // filter error
        let event = match event {
            Ok(event) => event,
            Err(e) => {
                tracing::error!("event error: {}", e);
                continue;
            }
        };
        // filter window event
        let window_event = match event {
            Event::Window(window_event) => window_event,
            _ => continue,
        };

        if window_event.change != WindowChange::Focus {
            continue;
        }

        tracing::debug!("window event: {:?}", window_event);

        let focused_workspace = match find_focused_workspace(&mut conn).await? {
            Some(w) => w,
            None => continue,
        };
        let focused_node = window_event.container;
        // if focused node is changed, update opacity
        if focused_node.id != prev_focused_node.id {
            // set opacity of currently focused node to 1
            conn.run_command(format!("[con_id=\"{}\"] opacity 1", focused_node.id))
                .await?;
            // set opacity of previously focused node (now inactive node) to given value
            if prev_focused_workspace_num == focused_workspace.num {
                conn.run_command(format!(
                    "[con_id=\"{}\"] opacity {}",
                    prev_focused_node.id, &opacity
                ))
                .await?;
            }

            prev_focused_node = focused_node.clone();
            prev_focused_workspace_num = focused_workspace.num;
        }
    }

    Ok(())
}

async fn find_focused_workspace(conn: &mut I3) -> anyhow::Result<Option<Workspace>> {
    let workspaces = conn.get_workspaces().await?;
    let focused_workspace = workspaces.into_iter().find(|w| w.focused);
    Ok(focused_workspace.clone())
}

async fn find_focused_node(conn: &mut I3) -> anyhow::Result<Option<Node>> {
    fn find(node: &Node) -> Option<Node> {
        if node.focused {
            return Some(node.clone());
        }
        for child in node.nodes.iter() {
            if let Some(focused) = find(child) {
                return Some(focused.clone());
            }
        }
        None
    }

    let tree = conn.get_tree().await?;
    Ok(find(&tree))
}
