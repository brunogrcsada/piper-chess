import React, { useEffect, useState } from "react";
// Import styling for this component
import "./timer.css";

class GameTimer {
  /* This class is used as a template to instantiate any timers
       throughout the game */

  // Example Usage - 5 second timer
  //timer = new Timer(5, function () { console.log("Ran!") });

  constructor(duration, callfn, update) {
    this.timer = duration; // Duration of timer in seconds
    this.seconds = 0; // Stores seconds passed since timer starts
    this.callfn = callfn; // Callback function after timer ends
    this.update = update; // State function to update UI
  }

  /* requestAnimationFrame allows code to be executed whenever the screen
    is repainted, with a returned value containing milliseconds that have
    passed since the browser window was loaded. */

  count(callback) {
    if (!this.stopped) {
      let current = Math.floor(callback / 1000); // Get current seconds
      if (this.seconds !== current) this.timer--; // Decrease timer by 1 if seconds are different
      this.seconds = current; // Update seconds variable
      this.update(this.display(this.timer));
      if (this.timer)
        requestAnimationFrame(this.count.bind(this)); // Request frames again
      else {
        this.callfn();
      } // When timer ends, run the callback method
    }
  }

  display(seconds) {
    return (
      (seconds - (seconds %= 60)) / 60 + (9 < seconds ? ":" : ":0") + seconds
    );
  }

  start() {
    this.stopped = false; // Resume/start timer
    // Request count method to be executed in the next screen repaint
    requestAnimationFrame(this.count.bind(this));
  }

  stop() {
    // Method used to stop timer from continuing
    this.stopped = true;
  }
}

function Timer({ set }) {
  const [time, setTime] = useState([]);

  // Only start the timer once when page loads
  useEffect(() => {
    const timer = new GameTimer(
      600,
      () => {
        console.log("Finished!!");
        set(true);
      },
      setTime
    );

    timer.start();
  }, [set]);

  return (
    <div className="timer">
      <span>{time}</span>
    </div>
  );
}

export default Timer;
