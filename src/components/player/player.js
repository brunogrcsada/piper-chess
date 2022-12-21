// Import styling for this component
import "./player.css";

function Player({ enabled, name, missing }) {
  return (
    <div className="player">
      <img
        alt={name}
        src={`gameplay/${enabled ? "enabled" : "disabled"}.svg`}
      ></img>
      <span className="name">{name}</span>

      {missing.map((item, index) => (
        <img
          key={index}
          alt={item}
          className={"lost"}
          src={`pieces/${item}.svg`}
        ></img>
      ))}
    </div>
  );
}

export default Player;
