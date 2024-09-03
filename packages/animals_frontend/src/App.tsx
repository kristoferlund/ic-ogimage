import Detail from "./Detail";
import List from "./List";
import NotFound from "./NotFound";

function Routes() {
  if (window.location.pathname === "/") {
    return <List />;
  }

  const pathname = window.location.pathname.split("/")[1];
  const id = Number.parseInt(pathname, 10);

  if (Number.isNaN(id)) {
    return <NotFound />;
  }

  return <Detail id={id} />;
}

function App() {
  return (
    <main>
      <Routes />
      <div className="column">
        <p style={{ textAlign: "center" }}>
          Smart contract canisters on the Internet Computer can serve content
          over HTTPS. That means whole websites can be hosted on the IC. This
          project demonstrates how to{" "}
          <b>dynamically generate Open Graph images</b> for an IC project. It
          also shows how to <b>dynamically modify the HTML metadata</b> of each
          page to improve SEO.
        </p>
        <p style={{ textAlign: "center" }}>
          <a href="https://github.com/kristoferlund/ic-ogimage">
            https://github.com/kristoferlund/ic-ogimage
          </a>
        </p>
        <div className="links">
          <a href="https://github.com/kristoferlund/ic-ogimage" target="_blank">
            <img src="https://img.shields.io/github/license/kristoferlund/ic-ogimage" />
          </a>

          <a href="https://github.com/kristoferlund/ic-ogimage" target="_blank">
            <img src="https://img.shields.io/github/stars/kristoferlund/ic-ogimage" />
          </a>
          <a href="https://github.com/kristoferlund" target="_blank">
            <img src="https://img.shields.io/github/followers/kristoferlund" />
          </a>
        </div>
      </div>
    </main>
  );
}

export default App;
