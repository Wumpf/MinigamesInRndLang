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
  PADDLE_SPEED = 0.8
  BALL_START_SPEED_MIN = 0.8
  BALL_START_SPEED_SPAN = 0.8 -- max - min
  BALL_RADIUS = 0.02

  player0Y = 0.5
  player1Y = 0.5
  
  player0Score = 0
  player1Score = 0

  love.graphics.setNewFont("PICHABS_.ttf", 100)

  respawnBall()
end

function respawnBall()
  ballPos = Vector(0.5, 0.5)
  local angle = math.random() * math.pi * 2;
  ballVel = Vector(math.sin(angle), math.cos(angle))
  ballVel = ballVel * (math.random() * BALL_START_SPEED_SPAN + BALL_START_SPEED_MIN)
end

function playerMovement(dt)
  if love.keyboard.isDown("w") then
    player0Y = player0Y - dt * PADDLE_SPEED
  end
  if love.keyboard.isDown("s") then
    player0Y = player0Y + dt * PADDLE_SPEED
  end
  player0Y = math.min(math.max(player0Y, PADDLE_SIZE.y / 2), 1 - PADDLE_SIZE.y / 2)

  if love.keyboard.isDown("up") then
    player1Y = player1Y - dt * PADDLE_SPEED
  end
  if love.keyboard.isDown("down") then
    player1Y = player1Y + dt * PADDLE_SPEED
  end
  player1Y = math.min(math.max(player1Y, PADDLE_SIZE.y / 2), 1 - PADDLE_SIZE.y / 2)
end

function ballPlayerCollision()
  local player0Upper = Vector(PLAYER0_X + PADDLE_SIZE.x / 2, player0Y + PADDLE_SIZE.y / 2)
  local player0Lower = Vector(PLAYER0_X + PADDLE_SIZE.x / 2, player0Y - PADDLE_SIZE.y / 2)
  local player1Upper = Vector(PLAYER1_X - PADDLE_SIZE.x / 2, player1Y + PADDLE_SIZE.y / 2)
  local player1Lower = Vector(PLAYER1_X - PADDLE_SIZE.x / 2, player1Y - PADDLE_SIZE.y / 2)

  if ballPos.x - BALL_RADIUS < player0Upper.x then
    if (ballPos.y < player0Upper.y and ballPos.y > player0Lower.y) or -- hit on surface
        (ballPos - player0Upper):lenSq() < BALL_RADIUS*BALL_RADIUS  or -- hit on edge
        (ballPos - player0Lower):lenSq() < BALL_RADIUS*BALL_RADIUS
    then
      local speed = math.abs(ballPos.y - player0Y) / PADDLE_SIZE.y * BALL_START_SPEED_SPAN + BALL_START_SPEED_MIN
      ballVel = (ballPos - Vector(PLAYER0_X, player0Y)):normalized() * speed
    end

  elseif ballPos.x + BALL_RADIUS > player1Upper.x then
    if (ballPos.y < player1Upper.y and ballPos.y > player1Lower.y) or -- hit on surface
        (ballPos - player1Upper):lenSq() < BALL_RADIUS*BALL_RADIUS  or -- hit on edge
        (ballPos - player1Lower):lenSq() < BALL_RADIUS*BALL_RADIUS
    then
      local speed = math.abs(ballPos.y - player1Y) / PADDLE_SIZE.y * BALL_START_SPEED_SPAN + BALL_START_SPEED_MIN
      ballVel = (ballPos - Vector(PLAYER1_X, player1Y)):normalized() * speed
    end
  end
end

function ballBorderCollision()
  local ballMin = Vector(ballPos.x - BALL_RADIUS, ballPos.y - BALL_RADIUS)
  local ballMax = Vector(ballPos.x + BALL_RADIUS, ballPos.y + BALL_RADIUS)

    -- Hit upper/lower border
  if ballMin.y < 0 then
    ballVel.y = -ballVel.y
    ballPos.y = BALL_RADIUS
  elseif ballMax.y > 1 then
    ballVel.y = -ballVel.y
    ballPos.y = 1 - BALL_RADIUS
  end
    -- Ball outside
  if ballMin.x < 0 then
    player1Score = player1Score + 1
    respawnBall()
  elseif ballMin.x > 1 then
    player0Score = player0Score + 1
    respawnBall()
  end
end

function love.update(dt)
  playerMovement(dt)
  
  -- Move ballPos
  ballPos = ballPos + ballVel * dt

  ballBorderCollision()
  ballPlayerCollision()
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
  local ballPixPos = relCorToAbs(ballPos)
  love.graphics.circle("fill", ballPixPos.x - ballRadiusPix / 2, ballPixPos.y - ballRadiusPix / 2, ballRadiusPix, 16)
end

function drawBackground()
  love.graphics.setColor(0, 180, 0)
  love.graphics.printf(player0Score, love.window.getWidth() * 0.25, love.window.getHeight() / 2 - 100, 0, 'center')
  love.graphics.printf(player1Score, love.window.getWidth() * 0.75, love.window.getHeight() / 2 - 100, 0, 'center')

  love.graphics.setLineWidth(10)
  love.graphics.setColor(150, 150, 150)
  for y = 10, love.window.getHeight(), 50 do
    love.graphics.line(love.window.getWidth() / 2, y, love.window.getWidth() / 2, y + 25)
  end
  love.graphics.setColor(255, 255, 255)
end

function love.draw()
  drawBackground()
  drawPaddles()
  drawBall()
end