#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec4 color;

layout(location = 0) out vec4 v_color;

layout(set = 0, binding = 0) uniform descriptor_camera_block {
	mat4 view;
	mat4 projection;
} descriptor_camera;

layout(set = 1, binding = 1) uniform descriptor_model_block {
	mat4 model;
	vec4 color;
} descriptor_model;

void main() {
	mat4 world_view =
		descriptor_camera.view *
		descriptor_model.model;
	vec4 screen_view =
		descriptor_camera.projection *
		world_view *
		vec4(position, 0.0, 1.0);
	v_color = color * descriptor_model.color;
	gl_Position = screen_view;
}