/*
 * @author Dave Kwiatkowski
 * codepen: @davekwiatkowski
 */

(function() {
	const SZ = 64,
		SPEED = 0.06,
		FREE_SPEED = SPEED / 40.0;
	var width, height, canvas, $, pts, xo, yo, xRot, zRot,
		time, left, right, up, down, lastzr, lastxr;

	init = function() {
		time = 0;
		xRot = Math.PI / 4;
		zRot = Math.PI / 4;
		lastxr = true;
		lastzr = true;
		pts = [];
		addPts();
	};
	addPt = function(x, y, z) {
		pts.push([x, y, z]);
	};
	addPts = function() {
		addPt(SZ, SZ, SZ);
		addPt(-SZ, SZ, SZ);
		addPt(-SZ, -SZ, SZ);
		addPt(SZ, -SZ, SZ);

		addPt(SZ, SZ, SZ);
		addPt(SZ, SZ, -SZ);
		addPt(-SZ, SZ, -SZ);
		addPt(-SZ, -SZ, -SZ);

		addPt(SZ, -SZ, -SZ);
		addPt(SZ, SZ, -SZ);
		addPt(SZ, -SZ, -SZ);
		addPt(SZ, -SZ, SZ);

		addPt(-SZ, -SZ, SZ);
		addPt(-SZ, -SZ, -SZ);
		addPt(-SZ, SZ, -SZ);
		addPt(-SZ, SZ, SZ);
	};
	to2d = function(x, y, z) {
		var xCos = Math.cos(xRot);
		var zCos = Math.cos(zRot);
		var xSin = Math.sin(xRot);
		var zSin = Math.sin(zRot);

		var xp = x * zCos - y * zSin;
		var yy = y * zCos + x * zSin;
		var yp = yy * xCos + z * xSin;
		return [xp, yp];
	};
	addVertices = function() {
		var pix = [pts.length];
		for (i = 0; i < pts.length; i++) {
			var x = pts[i][0];
			var y = pts[i][1];
			var z = pts[i][2];
			var pt2d = to2d(x, y, z);
			var x2d = pt2d[0] + xo;
			var y2d = pt2d[1] + yo;

			var r = x / SZ * 55 + 200;
			var g = y / SZ * 55 + 200;
			var b = z / SZ * 55 + 200;
			var a = 1;
			$.strokeStyle = 'rgba(' +
				r + ',' + g + ',' + b + ',' + a + ')';
			$.beginPath();
			$.moveTo(x2d, y2d);
			$.arc(x2d, y2d, 4, 0, 2 * Math.PI);
			$.stroke();
			pix.push(pt2d);
		}
		return pix;
	};
	addEdges = function(pix) {
		$.strokeStyle = 'rgba(255,255,255,0.2)';
		for (i = 0; i < pix.length - 1; i++) {
			var x0 = pix[i][0] + xo;
			var y0 = pix[i][1] + yo;
			var xf = pix[i + 1][0] + xo;
			var yf = pix[i + 1][1] + yo;

			$.beginPath();
			$.moveTo(x0, y0);
			$.lineTo(xf, yf);
			$.stroke();
		}
	};
	renderCube = function() {
		$.lineWidth = 8;
		$.clearRect(0, 0, width, height);
		var pix = addVertices();
		addEdges(pix);
	};
	render = function() {
		if (up) {
			zRot += SPEED;
			lastzr = true;
		}
		if (down) {
			zRot -= SPEED;
			lastzr = false;
		}
		if (left) {
			xRot += SPEED;
			lastxr = true;
		}
		if (right) {
			xRot -= SPEED;
			lastxr = false;
		}
		if (!up && !down && !left && !right) {
			xRot += (lastxr) ? FREE_SPEED : -FREE_SPEED;
			zRot += (lastzr) ? FREE_SPEED : -FREE_SPEED;
		}
		renderCube();
		time++;
		window.requestAnimationFrame(render);
	};
	reconfig = function() {
		canvas = document.getElementsByTagName('canvas')[0];
		width = window.innerWidth;
		height = window.innerHeight;
		canvas.width = width;
		canvas.height = height;
		$ = canvas.getContext('2d');
		xo = width / 2;
		yo = height / 2;
	};
	listenKeysUp = function(e) {
		var event = e || window.event;
		if (event.keyCode === 38) right = false;
		else if (event.keyCode === 40) left = false;
		if (event.keyCode === 37) up = false;
		else if (event.keyCode === 39) down = false;
	};
	listenKeysDown = function(e) {
		var event = e || window.event;
		if (event.keyCode === 38) right = true;
		else if (event.keyCode === 40) left = true;
		if (event.keyCode === 37) up = true;
		else if (event.keyCode === 39) down = true;
	};
	window.onresize = reconfig;
	window.onkeydown = listenKeysDown;
	window.onkeyup = listenKeysUp;
	start = function() {
		reconfig();
		init();
		render();
	};
	start();

		window.focus();
}());
