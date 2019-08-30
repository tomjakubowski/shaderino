// Author:
// Title:

#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

void main() {
  vec2 st = gl_FragCoord.xy/u_resolution.xy;
  st.x *= u_resolution.x/u_resolution.y;

  vec3 color = vec3(0.);
  vec2 mouse_st = u_mouse/u_resolution;
  mouse_st.x *= u_resolution.x/u_resolution.y;

  float dist = distance(st, mouse_st);
  float r = 0.3;
  float feather = 3.0 / u_resolution.x; // 3px?
  color = (1.0 - smoothstep(r, r+feather, dist)) * vec3(0.0, 1.0, 0.0);
  color += vec3(1.0, 0.0, 0.0);
  gl_FragColor = vec4(color,1.0);
}
