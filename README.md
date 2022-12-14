<div align="center">

  <img src="https://user-images.githubusercontent.com/54673205/208851651-b245c7ed-fe45-4561-b9fb-4bb18539c167.png" width="200" height="200">

  <h1>Piper Chess</h1>
 
  <br/>
  A robust, speedy and simple Chess implementation, made with ❤️ using Rust (backend) and ReactJS (frontend)  <br/>
  
  <br/>
  
</div>

## 🗒️ Overview:

![image](https://user-images.githubusercontent.com/54673205/208852176-5295c198-3a90-4fdd-bd16-f924ada2bd13.png)

## :heavy_exclamation_mark: Prerequisites

This project requires npm to execute the files, so ensure that it is installed.

### 1. Ensure node and npm are installed by running the following commands in your terminal:

```sh
node -v
```

```sh
npm -v
```

If they are not installed, follow the steps on [npm Docs](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).

### 2. To run end-to-end tests, please install 'serve' globally in your machine:

```sh
npm install -g serve
```

If you are getting some errors after using that command and you are on Linux/MacOS, try running it as a superuser (sudo)

### 3. You will also need Cargo, the package manager for Rust. It's a pretty neat tool!

#### The following command will work on Linux and macOS systems:

```sh
curl https://sh.rustup.rs -sSf | sh
```

#### If you are using Windows, please click on the following link to download rustup:

[https://win.rustup.rs](https://win.rustup.rs)

#### Further documentation:

[https://doc.rust-lang.org/cargo/getting-started/installation.html](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### 4. Afterwards, clone this repo:

```sh
git clone https://github.com/brunogrcsada/piper-chess
```

Now, you should have everything that you need to proceed! Navigate into the folder you just cloned to find the code :)

## :book: Getting Started

### Docker

You can run a single docker image for the back and frontend with the following commands:

```sh
docker build -t piper .
docker run -p 2020:2020 -p 3001:3001 piper
```

### 0. Before trying out any of the other commands:

```sh
npm install
```

### 1. Starting the Rust Actix web server:

#### For the purposes of this exercise, the server code is stored in the 'server' folder

```sh
cd server
cargo run
```

### 2. In the project directory, you can run:

```sh
npm start
```

Runs the app in the development mode. :point_up_2: \
Open [http://localhost:3001](http://localhost:3001) to view it in your browser.

The page will reload when you make changes.

### 3. To run any tests written for the backend, run the following command:

```sh
cargo test
```

### 3. To run logic and UI tests for the app, run the following command:

```sh
npm test
```

```sh
a
```

This launches the test runner in the interactive watch mode. :point_up_2: \
Clicking on the 'a' key runs all tests (excluding End-to-End tests).
See the section about [running tests](https://facebook.github.io/create-react-app/docs/running-tests) for more information.

### 3. To view full tests and code coverage for the code:

```sh
npm test -- --coverage --watchAll=false
```

### 4. To run the E2E tests:

#### Note that the following command might be different depending on your Operating System:

```sh
npm run build
```

Builds the app for production to the `build` folder. :point_up_2: \
It correctly bundles React in production mode and optimizes the build for the best performance.

See the section about [deployment](https://facebook.github.io/create-react-app/docs/deployment) for more information.

```sh
cd server
cargo start
```

```sh
PORT=4221 serve -s build & npm run test:e2e
```

## 🗒️ Challenge Outline

The main aim for this project is to create a 2-player chess game in a full stack application, with React being used as a frontend framework and Rust as a backend. To ensure that the game choices, movements and general logic is validated, any actions in the frontend must go through the Rust API, process and return a relevant game update to refresh any states in React. For Piper Chess, the main proposed solution outline is as follows:

### Project breakdown summary

When the page first loads, the game will attempt to retrieve an existing world status from the API. If successful, the board will auto-generate with all of the previous moves. Next, the current player attempts to move one of their pieces, so the front-end will give the hints on where they can move, and when the destination tile has been confirmed, an API call is made to the Actix backend to verify whether the move is valid in the first place, and then to update game state server-side if so. When the Actix backend processes the move, it returns a body with the updated game state, already with the next player selected and any special situations such as check or check mate verified. Using React’s nifty state management, an event is dispatched once the API data has been returned, and the board automatically re-renders its components, placing the pieces in the correct board co-ordinates. The same is repeated throughout the game, until the timer reaches 0, where the player with the highest number of stolen pieces wins, or when a player reaches check mate.

### Initial Work

Prior to developing extended logic for every chess piece and game component, I aimed to focus on creating the base project as a template for any further development. This included functionality for e2e and unit-based testing, as well as a base React and Rust project with some useful utilities that will be required based on the original plan.

The epic structure is quite split evenly between backend and front-end, so I focused on developing upon specific user stories full stack, rather than completing the frontend first and then the backend stories.

However, there is an exception to this; the work on piece logic was quite fragmented between the two parts of the stack. To ensure that I had a good idea on how to handle the pieces and how they behaved on the board, I completed initial frontend logic for pieces first. The Rust-based iteration is an improvement upon the work that I did to refactor and automate the rules for each piece, ending up in a co-ordinate based system for the API.

To ensure that the base API responses are valid throughout the development of the full stack application, I added a mock file with the initial global state of a game. Whenever the test is run, a comparison is made between the server response and the expected mock result. This can save quite a few headaches!

### Logic Diagrams

This is a rough design of the architectural implementation for this project. It includes the necessary basic calls to the API, including initial game state and update game board based on user actions. Board and timer class + method definitions are included as a visual aid. Similarly, each individual board piece, player and tile would be represented as their own objects.

![Advanced Programming drawio](https://user-images.githubusercontent.com/54673205/208896015-192ee6a0-d436-4563-b24b-20a7b6c9bcaa.png)

The following UML-focused diagrams show how each Rust implementation of functionality connects to each other in the project. These are the core components of the game board, any any following logic is undergone in ChessBoard operations

![Untitled drawio](https://user-images.githubusercontent.com/54673205/208897607-e9ec62a0-7b45-4496-afae-b60ad57e824e.png)


## Project Management

To ensure that all tasks for the fullstack game were completed on time, I planned out all required core components for both the React and Rust codebases, and placed them as cards in Trello. Whilst I was quite ambitious to implement web sockets with Actix, there were a lot of initial issues with setting it up from scratch in Rust; unfortunately, the deadline for the project has set as a future plan for the game.

![image](https://user-images.githubusercontent.com/54673205/208886015-c819b169-0b36-484a-adc5-aca5af3cd2eb.png)

## 🗒️ Coding Practices:

### Code cleanliness and documentation

Commenting code is a standard programming practice and mechanism to enhance comprehensibility whilst enabling developers to clarify the code in their own words. To ensure that my code is understood by another developer, I commented any unclear processes and included extra information behind my thought process. React helped me to compartmentalise functionality in different components, such as creating a separate timer widget that dealt with the general game countdown. Here is an example of where I applied both documentation practices and OOP to improve the readability and extensibility of the component:

![image](https://user-images.githubusercontent.com/54673205/208884404-b128d206-0085-4af1-9b0f-29668e27548e.png)

### Code refactoring

Another fundamental element relating to development is the process of refactoring code. This describes the process of refining and improving pre-written code in such a way that does not alter its external function whilst mitigating the introduction of bugs. Refactoring enables written code to be more concise, improving its readability. One of the most thought-provoking parts of this project was handling states, possible moves and in-game highlight helpers for each piece in the game. It was initially becoming tedious to create selection statements for every possible move type and piece, such as the one seen below:

![image](https://user-images.githubusercontent.com/54673205/208888629-e9a9eb1d-42ff-463f-9ab7-7d451ab6a321.png)

To get around this problem in the frontend, I created a map of all pieces and a 'rule book' attached to each. The possible move ranges are all defined in one exported variable which can be used accross the project. After making this change and ensuring that all rules were looped and accessed directly from the map, it was a lot cleaner to make extra changes to edge-cases and piece-specific behaviour. For the pawn, since it requires quite a bit of logic depending on enemies surrounding the piece and its position, I implemented the behaviour in the screenshot above to handle the piece separately. However, this didn't impact the information obtained and used from the rule book. 

![image](https://user-images.githubusercontent.com/54673205/208888803-c4423b12-4954-4c87-b84e-eaa7d56ff6ad.png)
![image](https://user-images.githubusercontent.com/54673205/208891005-ef1c1091-ace3-4027-97b9-e270e777073a.png)


## Bug Hunting

thread 'actix-rt|system:0|arbiter:3' panicked at 'attempt to subtract with overflow', src/gameplay/board.rs:276:66

Came across this bug quite often whilst working on the backend. It was one of those errors that was not reproducible at every step of the way, just sometimes with certain pieces. Ensuring that the shift calculation occured within the match, rather than outside of the player match definition fixed those thread issues from occuring again:

(match self.at(shift.destination) {
                    Cells::Missing => {
                        shift.origin.x == shift.destination.x
                            && (shift.origin.y == shift.destination.y - 1
                                || shift.origin.y == 1 && shift.destination.y == 3)
