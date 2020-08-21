import { html, Router, Route, css } from './basis.mjs';
import LandingPage from './page/landing.mjs';

const styles = {
  header: css`
    background-color: hsl(230, 15%, 85%);
  `
};

const App = () => html`
  <header class=${styles.header}>
    데백 API 관리 도구
  </header>
  <${Router}>
    <${Route} path="/" component=${LandingPage} />
  </${Router}>
`;

export default App;