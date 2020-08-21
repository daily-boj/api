import { injectGlobal } from '../basis.mjs';

injectGlobal`
  html, body {
    padding: 0;
    margin: 0;
  }

  html, body, #root {
    width: 100vw;
    height: 100vh;
  }
`;