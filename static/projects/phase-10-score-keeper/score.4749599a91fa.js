var players = [];

function loadLocalStorage() {
    players = JSON.parse(localStorage.getItem('players'));
    console.log(players);

    if(players == null) {
        players = [];
    }

    var table = document.getElementById("scoreTable");
    players.forEach(player => {
        // Insert row
        let row = table.insertRow(-1);
        // Add cells
        let name = row.insertCell(0);
        let phase = row.insertCell(1);
        let points = row.insertCell(2);
        let add_points = row.insertCell(3);

        // Make name row header
        name.outerHTML = '<th scope="row">' + player['name'] + '</th>';
        phase.innerHTML = player['phaseHistory'][player['phaseHistory'].length - 1];
        console.log(player['phaseHistory']);
        console.log(phase.innerHTML);
        points.innerHTML = player['pointHistory'].reduce((a, b) => a + b, 0);
        // Add points input field
        add_points.innerHTML = '<input type="tel" maxlength="3" size="3" onkeypress="validate(event)" id="' +
            player["addPointsID"] +  '" />';
        // Add ids
        name.id = player['nameID'];
        phase.id = player['phaseID'];
        points.id = player['pointsID'];
    });
}

loadLocalStorage();


function playerAdd() {
    // Get table
    var table = document.getElementById("scoreTable");
    // Get number of rows
    var table_length = table.rows.length;
    // Insert row at the bottom of the table
    var row = table.insertRow(-1);
    // Add row cells
    var name = row.insertCell(0);
    var phase = row.insertCell(1);
    var points = row.insertCell(2);
    var add_points = row.insertCell(3);

    // Make id for new data
    row.id = "addName-" + table_length;

    // Add name input field
    name.outerHTML = '<th><input type="text" size="6" id="' + row.id + '-name' + '""></th>';
    // Add add name button
    phase.innerHTML = '<button type="button" class="btn btn-primary" onclick="finishPlayerAdd(\'' + row.id + '\')">Add Name</button>';

    // Make primary buttons invisible
    document.getElementById("addUser").style.display = "none";
    document.getElementById("completeRound").style.display = "none";
}

function finishPlayerAdd(row_id) {
    // Make primary buttons visible
    document.getElementById("addUser").style.display = "block";
    document.getElementById("completeRound").style.display = "block";

    // Get row where name is being added
    var row = document.getElementById(row_id);
    // Get name being added
    var name_value = document.getElementById(row_id + "-name").value;

    // Add player
    players.push({
        'name': name_value,
        'nameID': name_value + "Name",
        'phaseID': name_value + "Phase",
        'pointsID': name_value + "Points",
        'addPointsID': name_value + "AddPoints",
        'pointHistory': [],
        'phaseHistory': [],
        'color': 'rgb(' + rand() + ', ' + rand() + ', ' + rand() + ')'
    });

    // Get row cells
    var name = row.cells[0];
    var phase = row.cells[1];
    var points = row.cells[2];
    var add_points = row.cells[3];

    // Make name a row header
    name.outerHTML = '<th scope="row">' + name_value + '</th>';
    // Start on phase 1 with 0 points
    phase.innerHTML = 1;
    points.innerHTML = 0;
    // Add point input field
    add_points.innerHTML = '<input type="tel" maxlength="3" size="3" onkeypress="validate(event)" id="' +
        players[players.length - 1]["addPointsID"] +  '" />';

    // Add ids from players
    name.id = players[players.length - 1]["nameID"];
    phase.id = players[players.length - 1]["phaseID"];
    points.id = players[players.length - 1]["pointsID"];
}

// Only allow numeric data be inputted for score
function validate(evt) {
  var theEvent = evt || window.event;

  // Handle paste
  if (theEvent.type === 'paste') {
      key = event.clipboardData.getData('text/plain');
  } else {
  // Handle key press
      var key = theEvent.keyCode || theEvent.which;
      key = String.fromCharCode(key);
  }
  var regex = /[0-9]|\./;
  if( !regex.test(key) ) {
    theEvent.returnValue = false;
    if(theEvent.preventDefault) theEvent.preventDefault();
  }
}

function addPlayerScore(player) {
    // Get player's total points and current phase
    let current_points = document.getElementById(player['pointsID']);
    let current_phase = document.getElementById(player['phaseID']);
    // Get player's points for the round
    let round_points = parseInt(document.getElementById(player['addPointsID']).value);

    // If bad input or no input, ignore
    if(document.getElementById(player['addPointsID']).value.length == 0 || isNaN(round_points)) {
        return;
    }

    // If less than 50 points this round, they move up a phase
    if(round_points < 50) {
        current_phase.innerHTML = parseInt(current_phase.innerHTML) + 1;
    }
    // Increase total points
    current_points.innerHTML = parseInt(current_points.innerHTML) + round_points;

    // Reset points input field
    document.getElementById(player['addPointsID']).value = '';

    // Add to history
    player['pointHistory'].push(round_points);
    player['phaseHistory'].push(parseInt(current_phase.innerHTML));
    console.log(parseInt(current_phase.innerHTML))
    console.log(player['phaseHistory']);
}

function completeRound() {
    // For each player, calculate total points and phase number
    players.forEach(player => addPlayerScore(player));
    localStorage.setItem('players', JSON.stringify(players));
    addStatistics();
}

function rand() {
    return Math.floor(Math.random() * 255);
}

function addStatistics() {
    var pointsOverTime = document.getElementById('pointsOverTime').getContext('2d');
    var pointsOverTimeChart = new Chart(pointsOverTime, {
        type: 'line',
        data: {
            labels: Array.from({length: players[0]['pointHistory'].length}, (x, i) => i),
            datasets: players.map(player => {
                return {
                    label: player.name,
                    fill: false,
                    borderColor: player['color'],
                    data: player['pointHistory']
                };
            })
        },
        options: {
            title: {
                display: true,
                text: 'Points Over Time'
            }
        }
    });

    var phaseOverTime = document.getElementById('phaseOverTime').getContext('2d');
    var phaseOverTimeChart = new Chart(phaseOverTime, {
        type: 'line',
        data: {
            labels: Array.from({length: players[0]['phaseHistory'].length}, (x, i) => i),
            datasets: players.map(player => {
                return {
                    label: player.name,
                    fill: false,
                    borderColor: player['color'],
                    data: player['phaseHistory']
                };
            })
        },
        options: {
            title: {
                display: true,
                text: 'Phases Over Time'
            }
        }
    });

}

function clearTable() {
    // Remove all rows from table and local storage
    var choice = confirm("Are you sure?");
    if(choice == true) {
        console.log("Clearing table and cookies")
        localStorage.setItem('players', JSON.stringify([]));
        // Remove rows from table
        $('#scoreBody tr').remove();
        // Reset players
        players = [];
    }
}
