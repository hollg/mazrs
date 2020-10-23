import('mazrs_wasm').then((maze) => {
	const mazeSVG = maze.generate()

	document.querySelector('div#maze_container').innerHTML = mazeSVG
})
