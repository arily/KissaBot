console.log("hello world")
print("hello lua")
console.log(_VERSION)
console.log("bye")

local hello = EventNode("hello")

hello:addlistener(function ()
  console.log("hello event1")
end)

hello:addlistener(function ()
  console.log("hello event2")
end)

hello:invoke()