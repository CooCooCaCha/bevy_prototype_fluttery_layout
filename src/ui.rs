use bevy::prelude::*;
use std::any::Any;
use std::default::Default;

#[derive(Debug, Clone, Copy)]
pub struct UIContraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32
}


#[derive(Debug, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

#[derive(Component, Debug, Clone)]
pub struct RectTransform {
    pub position: Vec2,
    pub size: Size,
    pub depth: i32
}

impl Default for RectTransform {
    fn default() -> Self {
        RectTransform {
            position: Vec2::ZERO,
            size: Size { width: 0.0, height: 0.0 },
            depth: 0
        }
    }
}

pub trait UIStyle {
    fn as_any(&self) -> &dyn Any;

    fn layout(
        &self, 
        constraints: UIContraints,
        layout_node_query: &Query<(&UINode, Option<&Children>)>,
        update_node_query: &mut Query<(&UINode, &mut RectTransform), With<Parent>>,
        children: Option<&Children>
    ) -> Size;
}

#[derive(Component)]
pub struct UINode {
    pub style: Box<dyn UIStyle + Send + Sync>
}

pub struct Row {
    pub width: f32
}

impl UIStyle for Row {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn layout(
        &self, 
        constraints: UIContraints,
        layout_node_query: &Query<(&UINode, Option<&Children>)>,
        update_node_query: &mut Query<(&UINode, &mut RectTransform), With<Parent>>,
        children: Option<&Children>
    ) -> Size 
    {
        let mut width = self.width;
        let height = constraints.max_height;

        if width > constraints.max_width {
            width = constraints.max_width;
        }

        if width < constraints.min_width {
            width = constraints.min_width;
        }

        if let Some(inner_children) = children {
            let mut total_flex: f32 = 0.0;
            let mut offset: f32 = 0.0;
    
            // Calculate total flex
            for child in inner_children.iter() {
                if let Ok((child_node, child_children)) = layout_node_query.get(*child) {
                    let style = child_node.style.as_any();

                    total_flex += match style.downcast_ref::<Expanded>() {
                        Some(Expanded { flex }) => *flex,
                        None => 1.0
                    };
                }
            }

            // Layout and update children
            for child in inner_children.iter() {
                let mut child_size = Size { width: 0.0, height: 0.0 };

                if let Ok((child_node, child_children)) = layout_node_query.get(*child) {
                    let style = child_node.style.as_any();

                    let flex = match style.downcast_ref::<Expanded>() {
                        Some(Expanded { flex }) => *flex,
                        None => 1.0
                    };

                    let child_width = (width / total_flex) * flex;

                    child_size = child_node.style.layout(UIContraints {
                        min_width: child_width,
                        max_width: child_width,
                        min_height: height,
                        max_height: height
                    }, layout_node_query, update_node_query, child_children);
                }

                if let Ok((child_node, mut child_transform)) = update_node_query.get_mut(*child) {
                    child_transform.position = Vec2::new(offset - width / 2.0 + child_size.width / 2.0, 0.0);
                    child_transform.size = child_size;
                }

                if let Ok((child_node, child_children)) = layout_node_query.get(*child) {
                    let style = child_node.style.as_any();

                    let flex = match style.downcast_ref::<Expanded>() {
                        Some(Expanded { flex }) => *flex,
                        None => 1.0
                    };

                    let child_width = (width / total_flex) * flex;

                    offset += child_width;
                }
            }
        }

        Size { width, height }
    }
}


pub struct Expanded {
    pub flex: f32
}

impl UIStyle for Expanded {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn layout(
        &self, 
        constraints: UIContraints,
        layout_node_query: &Query<(&UINode, Option<&Children>)>,
        update_node_query: &mut Query<(&UINode, &mut RectTransform), With<Parent>>,
        children: Option<&Children>) -> Size 
    {

        if let Some(inner_children) = children {
            for child in inner_children.iter() {
                let mut child_size = Size { width: 0.0, height: 0.0 };
        
                if let Ok((child_node, child_children)) = layout_node_query.get(*child) {
                    child_size = child_node.style.layout(constraints, layout_node_query, update_node_query, child_children);
                }
        
                if let Ok((child_node, mut child_transform)) = update_node_query.get_mut(*child) {
                    child_transform.position = Vec2::ZERO;
                    child_transform.size = child_size;
                }
            }
        }

        Size { width: constraints.max_width, height: constraints.max_height }
    }
}

pub struct Padding {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32
}

impl UIStyle for Padding {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn layout(
        &self, 
        constraints: UIContraints,
        layout_node_query: &Query<(&UINode, Option<&Children>)>,
        update_node_query: &mut Query<(&UINode, &mut RectTransform), With<Parent>>,
        children: Option<&Children>) -> Size
    {
        // UIConstraints minus the padding amount.
        let padded_constraints = UIContraints {
            min_width: constraints.min_width - (self.left + self.right),
            max_width: constraints.max_width - (self.left + self.right),
            min_height: constraints.min_height - (self.top + self.bottom),
            max_height: constraints.max_height - (self.top + self.bottom),
        };

        // Styles are responsible for updating their children
        if let Some(inner_children) = children {
            for child in inner_children.iter() {
                let mut child_size = Size { width: 0.0, height: 0.0 };
        
                // Recursively ask child nodes how large they would like to be.
                if let Ok((child_node, child_children)) = layout_node_query.get(*child) {
                    child_size = child_node.style.layout(padded_constraints, layout_node_query, update_node_query, child_children);
                }
        
                // Take that info into consideration and update the position and size of the children.
                if let Ok((child_node, mut child_transform)) = update_node_query.get_mut(*child) {
                    child_transform.position = Vec2::new(
                        -(self.left + self.right) / 2.0 + self.left, 
                        -(self.top + self.bottom) / 2.0 + self.bottom
                    );
                    child_transform.size = child_size;
                }
            }
        }

        Size { width: constraints.max_width, height: constraints.max_height }
    }
}

pub fn update_layout(
    windows: Res<Windows>,
    mut root_node_query: Query<(&UINode, &mut RectTransform, Option<&Children>), Without<Parent>>,
    layout_node_query: Query<(&UINode, Option<&Children>)>,
    mut update_node_query: Query<(&UINode, &mut RectTransform), With<Parent>>
) {
    let window = windows.get_primary().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    for (root_node, mut root_transform, root_children) in root_node_query.iter_mut() {
        let size = root_node.style.layout(UIContraints {
            min_width: 0.0,
            max_width: window_width,
            min_height: 0.0,
            max_height: window_height
        }, &layout_node_query, &mut update_node_query, root_children);

        root_transform.position = Vec2::ZERO;
        root_transform.size = size;
    }
}

pub fn sync_rect_transform_system(windows: Res<Windows>, mut node_query: Query<(&RectTransform, &mut Transform), With<UINode>>) {
    let window = windows.get_primary().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    for (rect_transform, mut transform) in node_query.iter_mut() {
        transform.translation = Vec3::new(
            rect_transform.position.x, 
            rect_transform.position.y, 
            rect_transform.depth as f32
        );
    }
}