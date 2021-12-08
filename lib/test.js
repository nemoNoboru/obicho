const engine = require('../')

console.log(engine.display_help())
console.log(engine.display_animal_numbers("macaco"))

console.log("[Start] calling get_once_results_callback")

const t = engine.get_once_results_callback((data, animal) => {
    console.log("[Start Inside Callback] Running callback")
    console.log(data)
    console.log("animal do dia: ", animal)
    console.log("[End Inside Callback] Running Ended")
})

// for loop with console logs saying i can still work
for (let i = 0; i < 100; i++) {
    console.log(i)
}

console.log(t)