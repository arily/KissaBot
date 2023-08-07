console.log("hello world")
print("hello lua")

_stop = core.addeventloop(function()
	console.log("hello eventloop")
	local i = 0
	__stop = core.addeventloop(function()
		i = i + 1
		console.log(tostring(i))
		if i > 10 then
			__stop()
		end
	end)
	_stop()
end)

console.log(_VERSION)
console.log("bye")
