const engine = require('../')

console.log(engine.display_help())

engine.get_once_results_callback((data, animal) => {
    console.log(data)
    console.log("animal do dia: ", animal)
})