console.log("Sudoku!");

const websocket_protocol = location.protocol.startsWith('https') ? 'wss' : 'ws';
const websocket_uri = `${websocket_protocol}://${location.host}/websocket`;
var websocket_client = null;

var playerID = null;
var gameID = null;

var selected_x = -1;
var selected_y = -1;
var selected_value = -1;

const selected_shadow = "0 0 0.7em white";
var ding = new Audio("ding.mp3");

function resetFieldSelected() {
	selected_x = -1;
	selected_y = -1;
}

function resetAllSelected() {
	resetFieldSelected();
	selected_value = -1;
}

function setup_websocket()
{
	websocket_client = new WebSocket(websocket_uri);

	websocket_client.onopen = () => {

		// Get the player name for registration
		let registration_request = JSON.stringify(
			{
				"PlayerName" : document.getElementById("registerName").value
			}
		);

		// Send the registration request & log completion of connection setup
		websocket_client.send(registration_request);
		console.log("Done with websocket connection setup!");
	}

	websocket_client.onerror = (ev) => {
		console.log("An error occurded");
		console.log(ev);
	}

	websocket_client.onmessage = (ev) => {
		handle_websocket_message(ev.data);
	}

	websocket_client.onclose = () => {

		console.log('Disconnected')
		websocket_client = null
	}
}

function handle_websocket_message
(
	data
)
{

	ding.play();

	var handled = false;
	let parsed_data = JSON.parse(data);

	if (playerID == null && "PlayerID" in parsed_data)                          // Got a response to player registration request
	{
		playerID = parsed_data["PlayerID"];

		if (playerID == null)
		{
			websocket_client.close();
		}

		handled = true;
	}
	
	if ("Games" in parsed_data)                                                 // Update regarding list of games
	{
		showListOfGames(parsed_data);

		handled = true;
	}

	if ("JoinLeaveGameID" in parsed_data)
	{
		// Depending on the current value, either set or reset the GameID variable
		if (gameID == null)
		{
			gameID = parsed_data["JoinLeaveGameID"];
			document.getElementById("games").style.display = "none";
			document.getElementById("game").style.display = "block";
			document.getElementById("ready").style.display = "block";

			// Hide the board again because we don't want to show it yet
			document.getElementById("board").style.display = "none";
			document.getElementById("numbers").style.display = "none";
		}
		else
		{
			gameID = null;
			document.getElementById("games").style.display = "block";

			// Hide all the other stuff
			document.getElementById("game").style.display = "none";
			document.getElementById("ready").style.display = "none";
			
			document.getElementById("board").style.display = "none";
			document.getElementById("numbers").style.display = "none";
		}

		handled = true;
	}
	
	if ("State" in parsed_data)
	{
		// Update the state of the current game
		showGame(parsed_data);

		handled = true;
	}
	

	if (!handled)
	{
		console.log('Received some data:');
		console.log(data);
	}
}

// Register player with name via POST request
function registerPlayer() 
{

	if (websocket_client == null)
	{
		setup_websocket();
		console.assert(websocket_client != null);
	}

	// Hide the registration elements and show the games section
	document.getElementById("registration").style.display = "none";
	document.getElementById("games").style.display = "block";
}

function createGame() 
{
	// Decode the request as JSON and send it via the websocket
	JSONdata = JSON.stringify(
		{
			"PlayerID" : playerID,
			"GameName" : document.getElementById("gameName").value,
			"Difficulty" : Math.max(Math.ceil(document.getElementById("gameDifficulty").value), 0)
		}
	);

	websocket_client.send(JSONdata);	
}

function toggleGame(id) 
{
	// Construct the JSON message
	JSONdata = JSON.stringify(
		{
			"PlayerID" : playerID,
			"GameID" : {"value": id == -1 ? gameID["value"] : id}
		}
	);

	// Send the message
	websocket_client.send(JSONdata);
}

function toggleReadyGame() 
{
	JSONdata = JSON.stringify(
		{
			"ReadyPlayerID" : playerID,
			"GameID" : gameID
		}
	);

	websocket_client.send(JSONdata);

	if (document.getElementById("readyButton").innerHTML == "Ready")            // Sent to server that we are READY
	{
		// Change button text to "Unready" and hide leave button
		document.getElementById("readyButton").innerHTML = "Unready";
		document.getElementById("leaveButton").style = "none";
	}
	else                                                                        // Sent to the server that we are NOT READY
	{
		// Change button text to "Ready" and show leave button
		document.getElementById("readyButton").innerHTML = "Ready";
		document.getElementById("leaveButton").style = "block";
	}
}

function fieldClick(id) 
{

	if
	(
		selected_x != -1 && 
		selected_y != -1
	) 
	{
		document.getElementById(getFieldID()).style.boxShadow = "none";
	}

	x_readout = parseInt(id[1]);
	y_readout = parseInt(id[3]);
	
	if
	(
		selected_x == x_readout && 
		selected_y == y_readout
	)
	{
		document.getElementById(id).style.boxShadow = "none";
		selected_x = -1;
		selected_y = -1;
	}
	else
	{
		document.getElementById(id).style.boxShadow = selected_shadow;
		selected_x = x_readout;
		selected_y = y_readout;
	}
	
	sendMoveToServerIfAllSelected();
}

function numberClick(id) 
{

	if(selected_value != -1) 
	{
		document.getElementById("n" + selected_value).style.boxShadow = "none";
	}

	value_readout = parseInt(id[1]);

	if(selected_value == value_readout) 
	{
		selected_value = -1;
	}
	else
	{
		document.getElementById(id).style.boxShadow = selected_shadow;
		selected_value = value_readout;
	}

	sendMoveToServerIfAllSelected();
}

function sendMoveToServerIfAllSelected() 
{
	if 
	(
		selected_x != -1 && 
		selected_y != -1 && 
		selected_value != -1
	) 
	{
		sendMoveToServer();
		document.getElementById(getFieldID()).style.boxShadow = "none";
		resetFieldSelected();
	}
}

function getFieldID() 
{
	return "x" + selected_x + "y" + selected_y;
}

document.onkeydown = function(evt) 
{
	evt = evt || window.event;
	if(parseInt(evt.key) > 0 && parseInt(evt.key) < 10) 
	{
		if(selected_x != -1 && selected_y != -1) 
		{
			selected_value = parseInt(evt.key);
			document.getElementById(getFieldID()).style.boxShadow = "none";
			sendMoveToServer();
			resetAllSelected();
		}
	}
};

function sendMoveToServer() 
{
	JSONdata = JSON.stringify(
		{
			"PlayerID" : playerID,
			"GameID" : gameID,
			"Field" : 
				{
					"X" : selected_x,
					"Y" : selected_y,
					"Value" : selected_value,
					"Color" : ""
				}
		}
	);

	websocket_client.send(JSONdata);
}


function showListOfGames(games) 
{
	// Clear the existing table
	document.getElementById("gamesTableBody").innerHTML = "";

	// Insert the data into the table
	for (let index in games["Games"])
	{
		// Get the information about a specific game
		let game = games["Games"][index];

		// Insert a new row into the table
		var row = document.getElementById("gamesTableBody").insertRow(-1);

		// Fill the newly created row
		row.insertCell(0).innerHTML = game["CreatorName"];
		row.insertCell(1).innerHTML = game["GameName"];
		row.insertCell(2).innerHTML = game["Difficulty"];
		row.insertCell(3).innerHTML = row.insertCell(3).innerHTML = (game["ReadyPlayers"]).toString() + "/" + (game["TotalPlayers"]).toString();
		row.insertCell(4).innerHTML = "<button onClick='toggleGame(" + game["GameID"]["value"] + ")'>Join</button>";
	}
}

function showGame(game)
{

	console.log(game);

	if ("Message" in game)
	{
		document.getElementById("message").innerHTML = game["Message"];
	}

	if ("Fields" in game)
	{
		// Show the board (if we have the data)
		if (game["Fields"].length > 0)
		{
			document.getElementById("game").style.display = "block";
			document.getElementById("board").style.display = "block";
			document.getElementById("numbers").style.display = "block";
			document.getElementById("ready").style.display = "none";
		}

		// Fill the board
		for (let index in game["Fields"])
		{
			let field = game["Fields"][index];

			let value = parseInt(field["Value"]);
			document.getElementById(
				"x" + field["X"] 
				+ "y" + field["Y"]
			).innerHTML = (value == 0 ? "&nbsp;" : value);

			document.getElementById(
				"x" + field["X"] 
				+ "y" + field["Y"]
			).style.backgroundColor = "#" + field["Color"];
		}
	}

	if ("Players" in game)
	{
		// Show the list of players
		document.getElementById("playersTableBody").innerHTML = "";

		for (let index in game["Players"]) 
		{
			let player = game["Players"][index];

			var row = document.getElementById("playersTableBody").insertRow(-1);
			var playerNameCell = row.insertCell(0);
			
			playerNameCell.innerHTML = player["PlayerName"];
			playerNameCell.style.color = "#" + player["Color"];

			row.insertCell(1).innerHTML = player["Points"];
		}
	}
}