if (not WORK["warm"] == true)
then
  WORK["warm"] = true
  WORK["x"] = 300
  WORK["y"] = 200
  DRAW["rect"] = {
    "home",
    "toggle false",
    "move 300 200",
    "rotate 45",
    "scale 1 1",
    "color 255 255 255",
    "move -25 -25",
    "toggle true",
    "move 50 0",
    "move 0 50",
    "move -50 0",
    "move 0 -50"
  }
end
DRAW_DIRTY=true

DRAW["rect"][3] = "move ".. WORK["x"] .. " " .. WORK["y"]
DRAW["rect"][4] = "rotate ".. TIMER * 90
DRAW["rect"][5] = "scale ".. math.sin(TIMER) + 1.5 .. " " .. math.cos(TIMER) + 1.5
DRAW["rect"][6] = "color ".. math.floor(100 * math.sin(TIMER) + 128) .. " " .. math.floor(100 * math.cos(TIMER) + 128) .. " " .. math.floor(100 * math.sin(TIMER + 1) + 128)

if UP then
  WORK["y"] = WORK["y"] - 1
end
if DOWN then
  WORK["y"] = WORK["y"] + 1
end
if LEFT then
  WORK["x"] = WORK["x"] - 1
end
if RIGHT then
  WORK["x"] = WORK["x"] + 1
end
