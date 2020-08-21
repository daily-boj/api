import { html, render, css } from './basis.mjs';
import App from './App.mjs';

import './styles/reset.mjs';

render(
  html`
    <${App} />
  `,
  document.getElementById('root')
);