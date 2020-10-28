import('mazrs_wasm').then((maze) => {
	const mazeSVG = maze.generate(50, 50)

	document.querySelector('div#maze_container').innerHTML = mazeSVG
})
