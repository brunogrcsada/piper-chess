import React from "react";
import ReactDOM from "react-dom/client";
import ReactRouter from "./router/router.js";

import "./index.css";

const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(<ReactRouter />);

const reportWebVitals = (onPerfEntry) => {
  if (onPerfEntry && onPerfEntry instanceof Function) {
    import("web-vitals").then(({ getCLS, getFID, getFCP, getLCP, getTTFB }) => {
      getCLS(onPerfEntry);
      getFID(onPerfEntry);
      getFCP(onPerfEntry);
      getLCP(onPerfEntry);
      getTTFB(onPerfEntry);
    });
  }
};

export default reportWebVitals;
