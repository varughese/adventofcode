// did this in the developer console

let findMaxes = (choices) => choices.split(';').map(set => set.trim().split(',').map(cubes => { 
    const [number, color] = cubes.trim().split(' '); 
    return { number: Number(number), color } 
})).reduce((max, set) => {
    max.red = Math.max(max.red, set.find(v => v.color === 'red')?.number || 0)
    max.green = Math.max(max.green, set.find(v => v.color === 'green')?.number || 0)
    max.blue = Math.max(max.blue, set.find(v => v.color === 'blue')?.number || 0)
    return max
}, { red: 0, green: 0, blue: 0})

input.split('\n').map(line => {
    let [gameId, choices] = line.split(':')
    const id = Number(gameId.split(" ")[1].trim())
    const maxes = findMaxes(choices)
    const power = maxes.red * maxes.green * maxes.blue;
    return { id, valid, power, maxes }
}).reduce((acc, v) => acc += v.power, 0)
