// import init, { generate } from 'mazrs_wasm'

// // init.then(() => {
// const svg = generate()
// console.log(svg)
// // })

import('mazrs_wasm').then((maze) => {
	const mazeSVG = maze.generate()

	document.querySelector('div#maze_container').innerHTML = mazeSVG
})
