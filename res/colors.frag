// http://glslsandbox.com/e#56865.0
// FIXME: colors look different

#ifdef GL_ES
precision mediump float;
#endif
#define PI 3.14159265359
#extension GL_OES_standard_derivatives : enable

uniform float u_time;
uniform vec2 u_mouse;
uniform vec2 u_resolution;




vec3 colorA = vec3(0.149,0.141,0.912);
vec3 colorB = vec3(1.000,0.833,0.224);

float plot (vec2 st, float pct){
  return  smoothstep( pct-0.01, pct, st.y) -
    smoothstep( pct, pct+0.01, st.y);
}

void main() {
  vec2 st = gl_FragCoord.xy/u_resolution.xy;
  vec3 color = vec3(0.0);

  float x = st.x;

  // craete pct from x
  vec3 pct = vec3(x);

  pct.r = smoothstep(0.0,1.0, x);
  pct.g = sin(x*PI);
  pct.b = pow(x,0.5);

  color = mix(colorA, colorB, pct);

  // Plot transition lines for each channel
  //vec3 mix(vec3 x, vec3 y, vec3 a)
  color = mix(color,vec3(1.0,0.0,0.0),plot(st,pct.r));
  color = mix(color,vec3(0.0,1.0,0.0),plot(st,pct.g));
  color = mix(color,vec3(0.0,0.0,1.0),plot(st,pct.b));

  gl_FragColor = vec4(color,1.0);
}
