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
