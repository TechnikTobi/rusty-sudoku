console.log("Sudoku!");

var listClient = null;
var gameClient = null;

var playerID = "";
var gameID = "";

var selected_x = -1;
var selected_y = -1;
var selected_value = -1;

const selected_shadow = "0 0 0.7em white";

function resetFieldSelected() {
	selected_x = -1;
	selected_y = -1;
}

function resetAllSelected() {
	resetFieldSelected();
	selected_value = -1;
}

// const ding = new Audio("ding.mp3");

// Register player with name via POST request
function registerPlayer() {
	const request = new XMLHttpRequest();
	request.open("POST", "/app/register", true);
	request.setRequestHeader("Content-Type", "application/json");
	request.send(JSON.stringify({
		"PlayerName" : document.getElementById("registerName").value
	}));

	request.onreadystatechange = (event) => {
		if(request.readyState == 4) {
			const responseData = JSON.parse(request.responseText);
			playerID = responseData["Data"]["PlayerID"];

			if(listClient == null) {
				listClient = Stomp.over(new SockJS("/websocket"));
				listClient.connect({}, function (frame) {
					listClient.subscribe("/gamesList", function (message) {const data = message.body; newRefreshGames(data)});
				});
			}

			const gamesRequest = new XMLHttpRequest();
			gamesRequest.open("GET", "/app/getGamesList", true);
			gamesRequest.send();
			gamesRequest.onreadystatechange = (event) => {
				if(gamesRequest.readyState == 4) {
					console.log(gamesRequest.responseText);
					newRefreshGames(gamesRequest.responseText);
				}
			}

			document.getElementById("registration").style.display = "none";
			document.getElementById("games").style.display = "block";
		}
	}
}

function createGame() {
	const request = new XMLHttpRequest();
	request.open("POST", "/app/createGame", true);
	request.setRequestHeader("Content-Type", "application/json");
	JSONdata = JSON.stringify({
		"PlayerID" : playerID,
		"GameName" : document.getElementById("gameName").value,
		"Difficulty" : Math.max(Math.ceil(document.getElementById("gameDifficulty").value), 0)
	});
	request.send(JSONdata);
}

function newRefreshGames(data) {
	const json = JSON.parse(data);
	document.getElementById("gamesTableBody").innerHTML = "";
	for(let index in json["Data"]["Games"]) {
		let game = json["Data"]["Games"][index];
		var row = document.getElementById("gamesTableBody").insertRow(-1);
		row.insertCell(0).innerHTML = game["CreatorName"];
		row.insertCell(1).innerHTML = game["GameName"];
		row.insertCell(2).innerHTML = game["Difficulty"];
		row.insertCell(3).innerHTML = (game["ReadyPlayers"]).toString() + "/" + (game["TotalPlayers"]).toString();
		row.insertCell(4).innerHTML = "<button onClick='joinGame(" + game["GameID"] + ")'>Join</button>";
	}
}

function showGame(message) {
	// ding.play();
	const json = JSON.parse(message.body);
	console.log(json);
	if("Message" in json["Data"]) {
		document.getElementById("message").innerHTML = json["Data"]["Message"];
	}
	if("Fields" in json["Data"]) {
		document.getElementById("game").style.display = "block";
		for(let index in json["Data"]["Fields"]) {
			let field = json["Data"]["Fields"][index];
			let value = parseInt(field["Value"]);
			document.getElementById("x" + field["X"] + "y" + field["Y"]).innerHTML = (value == 0 ? "&nbsp;" : value);
			document.getElementById("x" + field["X"] + "y" + field["Y"]).style.backgroundColor = "#" + field["Color"];
		}
		document.getElementById("playersTableBody").innerHTML = "";
		for(let index in json["Data"]["Players"]) {
			let player = json["Data"]["Players"][index];
			var row = document.getElementById("playersTableBody").insertRow(-1);
			var playerNameCell = row.insertCell(0);
			playerNameCell.innerHTML = player["PlayerName"];
			playerNameCell.style.color = "#" + player["Color"];
			row.insertCell(1).innerHTML = player["Points"];
		}
	}
}

function joinGame(id) {
	gameID = id.toString();
	const request = new XMLHttpRequest();
	request.open("POST", "/app/game/" + gameID + "/join", true);
	request.setRequestHeader("Content-Type", "application/json");
	JSONdata = JSON.stringify({
		"PlayerID" : playerID,
		"GameID" : gameID
	});
	request.send(JSONdata);

	request.onreadystatechange = (event) => {
		if(request.readyState == 4) {
			if(gameClient == null) {
				gameClient = Stomp.over(new SockJS("/websocket"));
				gameClient.connect({}, function (frame) {
					gameClient.subscribe("/game/" + gameID + "/update", function (message) {showGame(message)});
				});
			}

			document.getElementById("games").style.display = "none";
			document.getElementById("ready").style.display = "block";
		}
	}
}

function readyForGame() {
	const request = new XMLHttpRequest();
	request.open("POST", "/app/game/" + gameID + "/ready", true);
	request.setRequestHeader("Content-Type", "application/json");
	JSONdata = JSON.stringify({
		"PlayerID" : playerID,
		"GameID" : gameID
	});
	request.send(JSONdata);

	request.onreadystatechange = (event) => {
		if(request.readyState == 4) {
			document.getElementById("ready").style.display = "none";
		}
	}
}

function fieldClick(id) {
	if(selected_x != -1 && selected_y != -1) {
		document.getElementById(getFieldID()).style.boxShadow = "none";
	}
	x_readout = parseInt(id[1]);
	y_readout = parseInt(id[3]);
	if(selected_x == x_readout && selected_y == y_readout) {
		document.getElementById(id).style.boxShadow = "none";
		selected_x = -1;
		selected_y = -1;
	}else{
		document.getElementById(id).style.boxShadow = selected_shadow;
		selected_x = x_readout;
		selected_y = y_readout;
	}
	sendMoveToServerIfAllSelected();
}

function numberClick(id) {
	if(selected_value != -1) {
		document.getElementById("n" + selected_value).style.boxShadow = "none";
	}
	value_readout = parseInt(id[1]);
	if(selected_value == value_readout) {
		selected_value = -1;
	}else{
		document.getElementById(id).style.boxShadow = selected_shadow;
		selected_value = value_readout;
	}
	sendMoveToServerIfAllSelected();
}

function sendMoveToServerIfAllSelected() {
	if(selected_x != -1 && selected_y != -1 && selected_value != -1) {
		sendMoveToServer();
		document.getElementById(getFieldID()).style.boxShadow = "none";
		resetFieldSelected();
	}
}

function getFieldID() {
	return "x" + selected_x + "y" + selected_y;
}

document.onkeydown = function(evt) {
    evt = evt || window.event;
	if(parseInt(evt.key) > 0 && parseInt(evt.key) < 10) {
		if(selected_x != -1 && selected_y != -1) {
			selected_value = parseInt(evt.key);
			document.getElementById(getFieldID()).style.boxShadow = "none";
			sendMoveToServer();
			resetAllSelected();
		}
	}
};

function sendMoveToServer() {
	if(gameClient != null) {
		gameClient.send("/app/game/" + gameID + "/move", {}, JSON.stringify({
			"PlayerID" : playerID,
			"GameID" : gameID,
			"Field" : {
				"X" : selected_x,
				"Y" : selected_y,
				"Value" : selected_value,
				"Color" : ""
			}
		}));
	}
}
