require "vector"

function love.conf(t)
  t.window.title = "Pong"            -- The window title (string)
  t.window.width = 1024
  t.window.height = 786
  t.window.fsaa = 8                  -- The number of samples to use with multi-sampled antialiasing (number)
end


function relCorToAbs(cor)
  return Vector(cor.x * love.window.getWidth(), cor.y * love.window.getHeight())
end

function relLenToAbs(len)
  return len * love.window.getHeight()
end

function love.load()
  math.randomseed(os.time())

  PLAYER0_X = 0.05
  PLAYER1_X = 0.95
  PADDLE_SIZE = Vector(0.03, 0.2)
  PADDLE_SPEED = 0.5
  BALL_START_SPEED_MIN = 0.1
  BALL_START_SPEED_MAX = 0.2
  BALL_RADIUS = 0.02

  player0Y = 0.5
  player1Y = 0.5

  respawnBall()
end

function respawnBall()
  ballPos = Vector(0.5, 0.5)
  local angle = math.random() * math.pi * 2;
  ballVel = Vector(math.sin(angle), math.cos(angle))
  ballVel = ballVel * ((math.random() * (BALL_START_SPEED_MAX - BALL_START_SPEED_MIN)) + BALL_START_SPEED_MIN)
end

function love.update(dt)
  -- paddle up/down movement
  if love.keyboard.isDown("w") then
    player0Y = player0Y - dt * PADDLE_SPEED
  end
  if love.keyboard.isDown("s") then
    player0Y = player0Y + dt * PADDLE_SPEED
  end
  player0Y = math.min(math.max(player0Y, PADDLE_SIZE.x / 2), 1 - PADDLE_SIZE.x / 2)

  if love.keyboard.isDown("up") then
    player1Y = player1Y - dt * PADDLE_SPEED
  end
  if love.keyboard.isDown("down") then
    player1Y = player1Y + dt * PADDLE_SPEED
  end
  player1Y = math.min(math.max(player1Y, PADDLE_SIZE.x / 2), 1 - PADDLE_SIZE.y / 2)
  

  -- move ballPos
  ballPos = ballPos + ballVel * dt

end

function drawPaddles()
  local paddleSize = relCorToAbs(PADDLE_SIZE)
  
  local paddlePos = relCorToAbs(Vector(PLAYER0_X, player0Y))
  love.graphics.rectangle("fill", paddlePos.x - paddleSize.x / 2, paddlePos.y - paddleSize.y / 2, paddleSize.x, paddleSize.y)

  local paddlePos = relCorToAbs(Vector(PLAYER1_X, player1Y))
  love.graphics.rectangle("fill", paddlePos.x - paddleSize.x / 2, paddlePos.y - paddleSize.y / 2, paddleSize.x, paddleSize.y)
end

function drawBall()
  local ballRadiusPix = relLenToAbs(BALL_RADIUS)
  local ballPixPos = relCorToAbs(ballPos) - ballRadiusPix
  love.graphics.circle("fill", ballPixPos.x, ballPixPos.y, ballRadiusPix, 16)
end

function love.draw()
  drawPaddles()
  drawBall()
end