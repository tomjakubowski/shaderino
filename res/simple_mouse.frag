// Author:
// Title:

#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

void main() {
  vec3 color = vec3(0.);
  vec2 mnhtn_dist = abs(gl_FragCoord.xy - u_mouse);
  vec2 smooth_dist = smoothstep(vec2(0.5), vec2(1.5), abs(gl_FragCoord.xy - u_mouse));
  color = (1.0 - smooth_dist.x * smooth_dist.y) * vec3(0.0, 1.0, 0.0);
  gl_FragColor = vec4(color,1.0);
}
