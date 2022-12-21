import { useNavigate } from "react-router-dom";
import "./404.css";

function ErrorPage() {
  let navigate = useNavigate();

  return (
    <div className="Error" style={{ backgroundColor: "white" }}>
      <img className="notFoundImage" alt="logo" src="../images/404.webp"></img>
      <h1 style={{ color: "black" }}>
        Oops! Seems like this page.... doesn't exist?
      </h1>
      <button onClick={() => navigate("/")} className="goBack" type="button">
        Go back home!
      </button>
    </div>
  );
}

export default ErrorPage;
