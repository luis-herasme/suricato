use std::collections::VecDeque;

use glam::{Quat, Vec3};
use gltf::Gltf;

use crate::transform::Transform3D;

#[derive(Debug, Clone)]
struct Node {
    parent_index:        Option<usize>,
    children_index_list: Vec<usize>,
    local_transform:     Transform3D,
    global_transform:    Transform3D,
}

enum Interpolation {
    Linear,
}

struct Sampler {
    times:         Vec<f32>,
    values:        SamplerValues,
    interpolation: Interpolation,
}

impl Sampler {
    fn get_time_index(&self, time: f32) -> Option<usize> {
        let mut index = 0;

        while (index < self.times.len() - 1) && (time > self.times[index + 1]) {
            index = index + 1;
        }

        if index + 1 >= self.times.len() {
            return None;
        }

        return Some(index);
    }
}

enum SamplerValues {
    Vec3(Vec<Vec3>),
    Quat(Vec<Quat>),
}

struct Channel {
    sampler_index:        usize,
    target_node_index:    usize,
    target_node_property: NodeProperty,
}

enum NodeProperty {
    Translation,
    Rotation,
    Scale,
}

pub struct Animation {
    current_time:       f32,
    animation_duration: f32,

    nodes:    Vec<Option<Node>>,
    samplers: Vec<Sampler>,
    channels: Vec<Channel>,
}

impl Animation {
    pub fn update(&mut self, delta_time: f32) {
        self.current_time = (self.current_time + delta_time) % self.animation_duration;
        self.update_local_transforms();
        self.update_global_transform();
    }

    fn update_local_transforms(&mut self) {
        for channel in &self.channels {
            let node = self.nodes[channel.target_node_index].as_mut().unwrap();
            let sampler = &self.samplers[channel.sampler_index];

            let Some(index) = sampler.get_time_index(self.current_time) else {
                return;
            };

            let prev_time = sampler.times[index];
            let next_time = sampler.times[index + 1];

            let t = (self.current_time - prev_time) / (next_time - prev_time);

            match sampler.interpolation {
                Interpolation::Linear => {
                    match &sampler.values {
                        SamplerValues::Vec3(values) => {
                            let prev_value = values[index];
                            let next_value = values[index + 1];
                            let value = prev_value.lerp(next_value, t);

                            match &channel.target_node_property {
                                NodeProperty::Translation => {
                                    node.local_transform.translation = value;
                                }
                                NodeProperty::Scale => {
                                    node.local_transform.translation = value;
                                }
                                _ => panic!("Invalid sampler type"),
                            }
                        }
                        SamplerValues::Quat(values) => {
                            let prev_value = values[index];
                            let next_value = values[index + 1];
                            let value = prev_value.slerp(next_value, t);

                            match &channel.target_node_property {
                                NodeProperty::Rotation => {
                                    node.local_transform.rotation = value;
                                }
                                _ => panic!("Invalid sampler type"),
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update_global_transform(&mut self) {
        // Node index -> Parent index
        let mut nodes_to_update: VecDeque<(usize, usize)> = VecDeque::new();

        let mut current_node_index = 0;
        for i in 0..self.nodes.len() {
            if let Some(node) = &self.nodes[i] {
                if node.parent_index.is_none() {
                    for child_index in &node.children_index_list {
                        nodes_to_update.push_back((*child_index, current_node_index));
                    }
                }
            }

            current_node_index += 1;
        }

        while let Some((node_index, parent_index)) = nodes_to_update.pop_front() {
            let parent_node = self.nodes[parent_index].as_mut().unwrap();
            let parent_node_global_transform = parent_node.global_transform.to_mat4();

            let node = self.nodes[node_index].as_mut().unwrap();

            node.global_transform = Transform3D::from(parent_node_global_transform * node.local_transform.to_mat4());

            for child_index in &node.children_index_list {
                nodes_to_update.push_back((*child_index, node_index));
            }
        }
    }

    pub fn get_lines(&mut self) -> Vec<Vec3> {
        // Node index -> Parent index
        let mut nodes_to_update: VecDeque<(usize, usize)> = VecDeque::new();

        let mut current_node_index = 0;
        for i in 0..self.nodes.len() {
            if let Some(node) = &self.nodes[i] {
                if node.parent_index.is_none() {
                    for child_index in &node.children_index_list {
                        nodes_to_update.push_back((*child_index, current_node_index));
                    }
                }
            }

            current_node_index += 1;
        }

        let mut lines = Vec::new();
        while let Some((node_index, parent_index)) = nodes_to_update.pop_front() {
            let node_translation = self.nodes[node_index].as_mut().unwrap().global_transform.translation.clone();
            let parent_node_translation = self.nodes[parent_index].as_mut().unwrap().global_transform.translation.clone();

            let parent_node = self.nodes[parent_index].as_mut().unwrap();

            if parent_node.parent_index.is_some() {
                lines.push(parent_node_translation);
                lines.push(node_translation);
            }

            let node = self.nodes[node_index].as_mut().unwrap();
            for child_index in &node.children_index_list {
                nodes_to_update.push_back((*child_index, node_index));
            }
        }

        lines
    }
}

impl From<Gltf> for Animation {
    fn from(gltf: Gltf) -> Animation {
        let number_of_nodes = gltf.nodes().len();
        let mut nodes: Vec<Option<Node>> = vec![None; number_of_nodes];

        let samplers: Vec<Sampler> = Vec::new();
        let channels: Vec<Channel> = Vec::new();

        let skin = gltf.skins().next().unwrap();

        let mut parent_children_list = Vec::new();

        for joint in skin.joints() {
            let (translation, rotation, scale) = joint.transform().decomposed();

            let local_transform = Transform3D {
                translation: Vec3::from(translation),
                rotation:    Quat::from_array(rotation),
                scale:       Vec3::from(scale),
            };

            let children_index_list: Vec<usize> = joint.children().map(|node| node.index()).collect();

            parent_children_list.push((joint.index(), children_index_list.clone()));

            let node = Node {
                parent_index:        None, // Populated later
                children_index_list: children_index_list,
                local_transform:     local_transform,
                global_transform:    Transform3D::new(), // Populated later
            };

            nodes[joint.index()] = Some(node);
        }

        for (parent_index, children_index_list) in parent_children_list {
            for child_index in children_index_list {
                nodes[child_index].as_mut().unwrap().parent_index = Some(parent_index)
            }
        }

        Animation {
            current_time: 0.0,
            animation_duration: 1.0,

            nodes,
            samplers,
            channels,
        }
    }
}
