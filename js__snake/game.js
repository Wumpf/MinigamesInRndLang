// Constants
var NUM_TILES_X = 40
var NUM_TILES_Y = 25
var TILE_WIDTH = 25
var TILE_HEIGHT = 25

var TARGET_FRAMETIME = 1.0 / 60.0

var APPLE_PLACEMENT_CHECK = 0.1
var APPLE_PROBABILITY = 0.08

var PLAYER_WIDTH = 5
var PLAYER_SPEED = 200
var PLAYER_GROWTH = 100


// Create a canvas
var canvas = document.createElement("canvas");
canvas.width = NUM_TILES_X * TILE_WIDTH;
canvas.height = NUM_TILES_Y * TILE_HEIGHT;
canvas.style = "border:solid 1px #000000;";
document.body.appendChild(canvas);
var context = canvas.getContext("2d");

// Setup
var apples = [];
var playerLines = [{ x: canvas.width / 2 - 100, y: canvas.height / 2 }, { x: canvas.width / 2, y: canvas.height / 2 }]
var timeSinceLastApple = APPLE_PLACEMENT_CHECK;
var currentgrowth = 0;
var gameOver = false;
var playTime = 0;

// Key press.
var keyPressCode = -1
document.onkeydown = function (modifier) {
  keyPressCode = modifier.keyCode
}

// Gamelogic updates.
var update = function (timeSinceLastFrame) {
  playTime += timeSinceLastFrame

  // Add apples.
  timeSinceLastApple += timeSinceLastFrame;
  if (timeSinceLastApple > APPLE_PLACEMENT_CHECK) {
    timeSinceLastApple -= APPLE_PLACEMENT_CHECK;
    if (Math.random() < APPLE_PROBABILITY) {
      apples.push({
        x: Math.random() * (NUM_TILES_X - 1) * TILE_WIDTH,
        y: Math.random() * (NUM_TILES_Y - 1) * TILE_HEIGHT
      });
    }
  }

  function sign(number) {
    return number ? number < 0 ? -1 : 1 : 0;
  }

  // Move player.
  var movement = PLAYER_SPEED * timeSinceLastFrame;

  // Head
  var xDir = sign(playerLines[0].x - playerLines[1].x);
  var yDir = sign(playerLines[0].y - playerLines[1].y);
  function newDir(x, y) {
    xDir = x;
    yDir = y;
    playerLines.unshift({ x: playerLines[0].x, y: playerLines[0].y });
  }
  if (keyPressCode == 37 && xDir == 0) { // left
    newDir(-1, 0);
  } else if (keyPressCode == 38 && yDir == 0) { // up
    newDir(0, -1);
  } else if (keyPressCode == 39 && xDir == 0) { // right
    newDir(1, 0);
  } else if (keyPressCode == 40 && yDir == 0) {  // down
    newDir(0, 1);
  }
  playerLines[0].x += xDir * movement;
  playerLines[0].y += yDir * movement;

  // Move player tail.
  if (currentgrowth <= 0) {
    while (true) {
      var xDir = sign(playerLines[playerLines.length - 2].x - playerLines[playerLines.length - 1].x);
      var yDir = sign(playerLines[playerLines.length - 2].y - playerLines[playerLines.length - 1].y);
      playerLines[playerLines.length - 1].x += xDir * movement;
      playerLines[playerLines.length - 1].y += yDir * movement;

      // Check if "overshot".
      var xDirNew = playerLines[playerLines.length - 2].x - playerLines[playerLines.length - 1].x;
      var yDirNew = playerLines[playerLines.length - 2].y - playerLines[playerLines.length - 1].y;
      if (sign(xDirNew) != xDir)
        movement = Math.abs(xDirNew);
      else if (sign(yDirNew) != yDir)
        movement = Math.abs(yDirNew);
      else
        break;

      // If overshot remove last segment and apply remaining movement to next segment.
      playerLines.pop();
    }
  }
  else
    currentgrowth -= movement;

  // Head check against canvas.
  if (playerLines[0].x < 0 || playerLines[0].x > canvas.width || playerLines[0].y < 0 || playerLines[0].y > canvas.height) {
    gameOver = true;
    return;
  }

  // Head checks against all other segments
  function segmentBox(segment) {
    var info = new Object()
    info.minX = Math.min(playerLines[segment].x, playerLines[segment - 1].x);
    info.minY = Math.min(playerLines[segment].y, playerLines[segment - 1].y);
    info.maxX = Math.max(playerLines[segment].x, playerLines[segment - 1].x);
    info.maxY = Math.max(playerLines[segment].y, playerLines[segment - 1].y);
    if (info.minX == info.maxX) {
      info.minX -= PLAYER_WIDTH / 2;
      info.maxX += PLAYER_WIDTH / 2;
    }
    else if (info.minY == info.maxY) {
      info.minY -= PLAYER_WIDTH / 2;
      info.maxY += PLAYER_WIDTH / 2;
    }
    return info;
  }
  var headSegment = segmentBox(1);
  for (var playerSegment = 3; playerSegment < playerLines.length; ++playerSegment) {
    curSegment = segmentBox(playerSegment);

    if (curSegment.minX < headSegment.maxX && curSegment.maxX > headSegment.minX &&
        curSegment.minY < headSegment.maxY && curSegment.maxY > headSegment.minY) {
      gameOver = true;
      return;
    }
  }

  // Eating. Brute force check with head position.
  for (var i = 0; i < apples.length; ++i) {
    if (apples[i].x < playerLines[0].x && apples[i].x + TILE_HEIGHT > playerLines[0].x &&
        apples[i].y < playerLines[0].y && apples[i].y + TILE_HEIGHT > playerLines[0].y) {
      apples[i] = apples[apples.length-1];
      apples.pop();
      currentgrowth += PLAYER_GROWTH;
    }
  }
}

// Draw everything.
var render = function (timeSinceLastFrame) {
  context.clearRect(0, 0, canvas.width, canvas.height);

  // Background
  context.fillStyle = "#1a1";
  context.fillRect(0, 0, canvas.width, canvas.height);

  // Apples
  context.fillStyle = "#d00";
  for (var i = 0; i < apples.length; ++i) {
    context.fillRect(apples[i].x, apples[i].y, TILE_WIDTH, TILE_HEIGHT);
  }

  // Player
  context.strokeStyle = "#00d";
  context.lineWidth = PLAYER_WIDTH;
  context.beginPath();
  context.moveTo(playerLines[0].x, playerLines[0].y);
  for (var i = 1; i < playerLines.length; ++i) {
    context.lineTo(playerLines[i].x, playerLines[i].y);
  }
  context.stroke();

  // Timer & length
  {
    context.fillStyle = "#fff";
    context.globalAlpha = 0.5;
    context.font = "bold 20px sans-serif";
    var minutes = playTime / 60;
    var seconds = playTime - Math.floor(minutes) * 60;
    var text = minutes.toFixed(0).concat(":", seconds >= 10 ? "" : "0", seconds.toFixed());
    var textSize = context.measureText(text);
    context.fillText(text, canvas.width - textSize.width - 20, canvas.height - 20);
    context.globalAlpha = 1.0;
  }

  // Game over screen
  if (gameOver) {
    context.fillStyle = "#fff";
    context.font = "bold 80px sans-serif";
    var textSize = context.measureText("Game Over");
    context.fillText("Game Over", (canvas.width - textSize.width) / 2, 200);
  }
}

// Gameloop.
var run = function () {
  var now = Date.now();
  var timeSinceLastFrame = (now - lastFrameTime) / 1000;    // duration in seconds

  if (timeSinceLastFrame > TARGET_FRAMETIME) {
    if(!gameOver) update(timeSinceLastFrame);
    render(timeSinceLastFrame);
    lastFrameTime = now;
  }

  requestAnimationFrame(run);
}

// Cross-browser support for requestAnimationFrame;
requestAnimationFrame = window.requestAnimationFrame || window.webkitRequestAnimationFrame || window.msRequestAnimationFrame || window.mozRequestAnimationFrame;

var lastFrameTime = Date.now();
run()