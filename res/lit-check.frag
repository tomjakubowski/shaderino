#version 110

// Author @patriciogv ( patriciogonzalezvivo.com ) - 2015
// (edits by Tom Jakubowski)

#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform float u_time;

#define PI 3.14159265358979323846

vec2 rotate2D(vec2 _st, float _angle){
  _st -= 0.5;
  _st =  mat2(cos(_angle),-sin(_angle),
      sin(_angle),cos(_angle)) * _st;
  _st += 0.5;
  return _st;
}

vec2 tile(vec2 st, float zoom){
  st *= zoom;
  return fract(st);
}

float tile_id(vec2 st, float zoom) {
  st *= zoom;
  return floor(st.x) + zoom * floor(st.y);
}

float box(vec2 _st, vec2 _size, float _smoothEdges){
  _size = vec2(0.5)-_size*0.5;
  vec2 aa = vec2(_smoothEdges*0.5);
  vec2 uv = smoothstep(_size,_size+aa,_st);
  uv *= smoothstep(_size,_size+aa,vec2(1.0)-_st);
  return uv.x*uv.y;
}

void main(void){
  vec2 st = gl_FragCoord.xy/u_resolution.xy;
  vec3 color = vec3(0.0);
  vec2 orig_st = st;

  // Divide the space in N
  const float div = 4.0;
  const float max_id = div * div;
  float id = tile_id(st, div);
  float lit_id = floor(mod(10.0 * u_time, max_id));
  st = tile(st, div);

  // Use a matrix to rotate the space 45 degrees
  st = rotate2D(st, PI*0.25);

  // Draw a square
  vec3 lit_color = vec3(0.0, 1.0, 0.0);
  vec3 fg_color = mix(lit_color, vec3(1.0), step(1.0, abs(id - lit_id)));
  color = box(st,vec2(0.7),0.01) * fg_color;

  gl_FragColor = vec4(color,1.0);
}
