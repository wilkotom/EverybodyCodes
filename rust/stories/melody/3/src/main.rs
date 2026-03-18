use std::error::Error;

use aochelpers::get_everybodycodes_input;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Connection {
    colour: String,
    shape: String
}

#[derive(Debug, Clone)]
struct NodeAttachment {
    connection_type: Connection,
    attachment: Option<Box<ScaleNode>>
}

#[derive(Debug, Clone)]
struct ScaleNode {
    id: i32,
    plug: Connection,
    left_socket: NodeAttachment,
    right_socket: NodeAttachment,
}

impl ScaleNode {

    fn walk_ids(&self) -> Vec<i32> {
        let mut ids = match &self.left_socket.attachment {
            None => Vec::new(),
            Some(scale_node) => scale_node.walk_ids(),
        };
        ids.push(self.id);
        if let Some(right) = &self.right_socket.attachment {
            ids.append(&mut right.walk_ids());
        }
        
        ids
    }

    fn attach_node(&mut self, other: Self) -> Option<ScaleNode> {
        
        if self.left_socket.connection_type == other.plug && self.left_socket.attachment.is_none() {
            self.left_socket.attachment = Some(Box::new(other));
            return None;
        }
        
        let unattached = if let Some(left_attachment) = self.left_socket.attachment.as_mut() {
                left_attachment.attach_node(other)
            } else {
                Some(other)
            };

        match unattached{
            Some(other) => {
                if self.right_socket.connection_type == other.plug && self.right_socket.attachment.is_none() {
                    self.right_socket.attachment = Some(Box::new(other));
                    None
                } else {
                    if let Some(right_attachment) = self.right_socket.attachment.as_mut() {
                        right_attachment.attach_node(other)
                    } else {
                        Some(other)
                    }
                }
            }
            None => {
                None
            }
        }
    }
    

    fn weak_attach_node(&mut self, other: Self) -> Option<ScaleNode> {

        if (self.left_socket.connection_type.colour == other.plug.colour || self.left_socket.connection_type.shape == other.plug.shape) && self.left_socket.attachment.is_none() {
            self.left_socket.attachment = Some(Box::new(other));
            return None;
        }
        
        let unattached = if let Some(left_attachment) = self.left_socket.attachment.as_mut() {
                left_attachment.weak_attach_node(other)
            } else {
                Some(other)
            };

        match unattached{
            Some(other) => {
                if( self.right_socket.connection_type.colour == other.plug.colour || self.right_socket.connection_type.shape == other.plug.shape) && self.right_socket.attachment.is_none() {
                    self.right_socket.attachment = Some(Box::new(other));
                    None
                } else {
                    if let Some(right_attachment) = self.right_socket.attachment.as_mut() {
                        right_attachment.weak_attach_node(other)
                    } else {
                        Some(other)
                    }
                }
            }
            None => {
                None
            }
        }

    }

    fn attach_node_displacing_weak(&mut self, mut other: Self) -> Option<ScaleNode> {
        if (self.left_socket.connection_type.colour == other.plug.colour || self.left_socket.connection_type.shape == other.plug.shape) && self.left_socket.attachment.is_none() {
            self.left_socket.attachment = Some(Box::new(other));
        
            return None;
        }

        if let Some(left_attachment) = &mut self.left_socket.attachment {
            if left_attachment.plug != self.left_socket.connection_type && self.left_socket.connection_type == other.plug {
                other = std::mem::replace(left_attachment, other);
            } else {
                let result = left_attachment.attach_node_displacing_weak(other);
                if let Some(node) = result {
                    other = node
                } else {
                    return None;
                }
            }
        }
        if (self.right_socket.connection_type.colour == other.plug.colour || self.right_socket.connection_type.shape == other.plug.shape) && self.right_socket.attachment.is_none() {
            self.right_socket.attachment = Some(Box::new(other));
        
            return None;
        }
        if let Some(right_attachment) = &mut self.right_socket.attachment {
            if right_attachment.plug != self.right_socket.connection_type && self.right_socket.connection_type == other.plug{
                other = std::mem::replace(right_attachment, other);
            } else {
                let result = right_attachment.attach_node_displacing_weak(other);
                if let Some(node) = result {
                    other = node
                } else {
                    return None;
                }

            }
        }


     Some(other)
    }

}

fn main() -> Result<(), Box<dyn Error>>{
    if let Ok(nodes) = parse_data(&get_everybodycodes_input(3, 3, 1)?) {
        println!("Part 1: {:?}", part1(nodes));
    }
    if let Ok(nodes) = parse_data(&get_everybodycodes_input(3, 3, 2)?) {
        println!("Part 2: {:?}", part2(nodes));
    }

    if let Ok(nodes) = parse_data(&get_everybodycodes_input(3, 3, 3)?) {
        println!("Part 3: {:?}", part3(nodes));
    }
    Ok(())
}

fn parse_data(data: &str) -> Result<Vec<ScaleNode>, Box<dyn Error>>{
    let mut scales = Vec::new();
    for line in data.lines() {
        let mut sections = line.split(',');
        let id = sections.next().unwrap_or_default().split('=').last().ok_or("No ID section")?.parse::<i32>()?;
        let plug = parse_connection(sections.next().unwrap_or_default())?;
        let left = NodeAttachment{connection_type: parse_connection(sections.next().unwrap_or_default())?, attachment: None};
        let right = NodeAttachment{connection_type: parse_connection(sections.next().unwrap_or_default())?, attachment: None};
        scales.push(ScaleNode { id, plug, left_socket: left, right_socket: right,  });
    }

    Ok(scales)
}

fn parse_connection(conn: &str) -> Result<Connection, Box<dyn Error>> {
    let mut words = conn.split('=').last().ok_or("No Separator Detected")?.split(" ");
    Ok(Connection{ 
        colour: words.next().ok_or("No colour Detected")?.into(),
        shape: words.next().ok_or("No shape Detected")?.into()
})

}

fn part1(nodes: Vec<ScaleNode>) -> i32 {
    let mut nodes: std::vec::IntoIter<ScaleNode> = nodes.into_iter();
    let mut root = nodes.next().unwrap();
    for node in nodes {
        root.attach_node(node);
    }
    root.walk_ids().iter().enumerate().map(|(index, id)| (index as i32 + 1) * id).sum()
}

fn part2(nodes: Vec<ScaleNode>) -> i32 {
    let mut nodes: std::vec::IntoIter<ScaleNode> = nodes.into_iter();
    let mut root = nodes.next().unwrap();
    for node in nodes {
        root.weak_attach_node(node);
    }
    root.walk_ids().iter().enumerate().map(|(index, id)| (index as i32 + 1) * id).sum()
}


fn part3(nodes: Vec<ScaleNode>) -> i32 {
    let mut nodes: std::vec::IntoIter<ScaleNode> = nodes.into_iter();
    let mut root = nodes.next().unwrap();
    for node in nodes {
        let mut unattached = Some(node);
        while let Some(n) = unattached {
            unattached = root.attach_node_displacing_weak(n);
        }
    }
    root.walk_ids().iter().enumerate().map(|(index, id)| (index as i32 + 1) * id).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = "id=1, plug=BLUE HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=BLUE PENTAGON, data=?
id=2, plug=GREEN CIRCLE, leftSocket=BLUE HEXAGON, rightSocket=BLUE CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=BLUE CIRCLE, data=?
id=4, plug=BLUE CIRCLE, leftSocket=RED HEXAGON, rightSocket=BLUE HEXAGON, data=?
id=5, plug=RED HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=RED HEXAGON, data=?";
        let nodes = parse_data(data).unwrap();
        let result = part1(nodes);
        assert_eq!(result, 43);
    }

        #[test]
    fn test_part2() {
        let data = "id=1, plug=RED TRIANGLE, leftSocket=RED TRIANGLE, rightSocket=RED TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=RED PENTAGON, leftSocket=GREEN CIRCLE, rightSocket=GREEN CIRCLE, data=?";
        let nodes = parse_data(data).unwrap();
        let result = part2(nodes);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part3() {
        let data = "id=1, plug=RED TRIANGLE, leftSocket=BLUE TRIANGLE, rightSocket=GREEN TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=BLUE TRIANGLE, leftSocket=GREEN CIRCLE, rightSocket=RED CIRCLE, data=?
id=6, plug=BLUE TRIANGLE, leftSocket=GREEN CIRCLE, rightSocket=RED CIRCLE, data=?";
        let nodes = parse_data(data).unwrap();
        let result = part3(nodes);
        assert_eq!(result, 60);
    }
}