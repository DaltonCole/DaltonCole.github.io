b = ['b0', 'b1']
mod = ['mod0', 'mod1']

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

// Add equation input
function addEquation() {
    let input_div = document.getElementById('input');
    let equation = document.createElement('div');
    equation.classList.add('col-12');
    equation.setAttribute('align', 'center')
    let text = document.createElement('p');
    text.innerHTML = '<p>x &#8801; <input type="tel" size="3" onkeypress="validate(event)" id="b' + b.length + '"/> (mod <input type="tel" size="3" onkeypress="validate(event)" id="mod ' + mod.length +'">)';
    b.push('b' + b.length);
    mod.push('mod' + mod.length);
    console.log(mod);

    // Add equation to input div
    equation.appendChild(text);
    input_div.appendChild(equation);
}

// Solve!
function calculate() {
    // Get inputs
    let bi = []
    let modi = []
    b.forEach(i => bi.push(parseInt(document.getElementById(i).value)));
    mod.forEach(i => modi.push(parseInt(document.getElementById(i).value)));

    // Do math
    let N = modi.reduce((a, b) => a*b);
    // Get the mods multipled together without the ith mod
    let Ni = Array.from(modi, x => N / x);
    // Find the inverse of Ni
    let xi = [];
    let coeff_array = [];
    for(let i = 0; i < mod.length; i++) {
        let coeff = Ni[i] - (Math.floor(Ni[i] / modi[i]) * modi[i]);
        coeff_array.push(coeff);
        for(let c = 1; c < modi[i]; c++) {
            if(((coeff * c) % modi[i]) == (1)) {
                xi.push(c)
                break;
            }
        }
    }
    // Multiply all columns together
    let bnx = [];
    for(let i = 0; i < mod.length; i++) {
        bnx.push(bi[i] * Ni[i] * xi[i]);
    }
    // Add all rows together
    let total = bnx.reduce((a, b) => a + b);
    // Answer
    let answer = total % N;

    // --- Table Time --- //
    // Get row
    let solve_div = document.getElementById('solve');
    solve_div.innerHTML = ''
    // Make column and add attributes
    let solve_col = document.createElement('div');
    solve_col.classList.add('col-12');
    solve_col.setAttribute('align', 'center');
    // Make table and add attributes
    let solve_table = document.createElement('table');
    solve_table.classList.add('table');
    // Make table header
    solve_table.createTHead();
    solve_table.tHead.innerHTML = '<tr><th>b<sub>i</sub></th><th>N<sub>i</sub></th><th>x<sub>i</sub></th><th>b<sub>i</sub>N<sub>i</sub>x<sub>i</sub></th></tr>';
    // Add rows
    for(let i = 0; i < modi.length; i++) {
        let row = solve_table.insertRow(i+1);
        row.insertCell(0).innerHTML = bi[i];
        row.insertCell(1).innerHTML = Ni[i];
        row.insertCell(2).innerHTML = xi[i];
        row.insertCell(3).innerHTML = bnx[i];
    }
    // Add total row
    let row = solve_table.insertRow(modi.length + 1);
    row.insertCell(0).innerHTML;
    row.insertCell(1).innerHTML;
    row.insertCell(2).innerHTML = 'Total: ';
    row.insertCell(3).innerHTML = total;

    // Final answer
    let final_answer_div = document.createElement('div');
    final_answer_div.classList.add('col-12');
    final_answer_div.setAttribute('align', 'center');
    let final_answer = document.createElement('b');
    final_answer.innerHTML = 'Final Answer: x &#8801; ' + answer + ' (mod ' + N + ')'

    // Add everything to DOM
    solve_col.appendChild(solve_table);
    solve_div.appendChild(solve_col);
    final_answer_div.appendChild(final_answer);
    solve_div.appendChild(final_answer_div);

    if(isNaN(answer)) {
        let para = document.createElement('b');
        let col_div = document.createElement('div');
        col_div.classList.add('col-12');
        col_div.setAttribute('align', 'center');
        para.innerHTML = 'NOTE: ALL MODS MUST BE CO-PRIME'
        col_div.appendChild(para);
        solve_div.appendChild(col_div);
    }

    // --- Show Work -- //
    // Bi column
    let para = document.createElement('p')
    para.innerHTML = 'Step 1 - Create b<sub>i</sub> column from remainders:'
    solve_div.appendChild(para);
    let bi_table = document.createElement('table');
    bi_table.classList.add('table');
    bi_table.createTHead();
    bi_table.tHead.innerHTML = '<tr><th>b<sub>i</sub></th></tr>'
    for(let i = 0; i < bi.length; i++) {
        let row = bi_table.insertRow(i+1);
        row.insertCell(0).innerHTML = bi[i];
    }
    solve_div.appendChild(bi_table);
    // Ni column
    para = document.createElement('p')
    para.innerHTML = 'Step 2 - Create N<sub>i</sub> column by multiplying all remainders together except the i<sup>th</sup> row:'
    solve_div.appendChild(para);
    let ni_table = document.createElement('table');
    ni_table.classList.add('table');
    ni_table.createTHead();
    ni_table.tHead.innerHTML = '<tr><th>b<sub>i</sub></th><th>N<sub>i</sub></th></tr>'
    for(let i = 0; i < Ni.length; i++) {
        let row = ni_table.insertRow(i+1);
        row.insertCell(0).innerHTML = bi[i];
        let tmp = '';
        for(let j = 0; j < Ni.length; j++) {
            tmp += modi[j] + ' * ';
        }
        row.insertCell(1).innerHTML = tmp.slice(0, tmp.length - 3) + ' / ' + modi[i] + ' = ' + Ni[i];
    }
    solve_div.appendChild(ni_table);
    // xi column
    para = document.createElement('p')
    para.innerHTML = 'Step 3 - Create x<sub>i</sub> column by finding the inverse of N<sub>i</sub>:'
    solve_div.appendChild(para);
    let xi_table = document.createElement('table');
    xi_table.classList.add('table');
    xi_table.createTHead();
    xi_table.tHead.innerHTML = '<tr><th>b<sub>i</sub></th><th>N<sub>i</sub></th><th>x<sub>i</sub></th></tr>'
    for(let i = 0; i < xi.length; i++) {
        let row = xi_table.insertRow(i+1);
        row.insertCell(0).innerHTML = bi[i];
        row.insertCell(1).innerHTML = Ni[i];
        row.insertCell(2).innerHTML = xi[i];
    }
    for(let i = 0; i < xi.length; i++) {
        let col = document.createElement('div')
        col.classList.add('col-12');
        let row = document.createElement('p');
        row.innerHTML = 'x<sub>' + (i+1) + '</sub>:';
        col.appendChild(row);
        row = document.createElement('p');
        row.innerHTML = Ni[i] + 'x<sub>' + (i+1) + '</sub> &#8801; 1 (mod ' + modi[i] + ') &rArr;';
        col.appendChild(row);
        row = document.createElement('p');
        row.innerHTML = Ni[i] + ' mod ' + modi[i] + ' = ' + coeff_array[i] + ' &rArr;';
        col.appendChild(row);
        row = document.createElement('p');
        row.innerHTML = coeff_array[i] + 'x<sub>' + (i+1) + '</sub> &#8801; 1 (mod ' + modi[i] + ') &rArr;'
        col.appendChild(row);

        row = document.createElement('p');
        row.innerHTML = '(' + coeff_array[i] + ' * <b>' + xi[i] + '</b>) mod ' + modi[i] + ' = 1 &rArr;';
        col.appendChild(row);

        row = document.createElement('p');
        row.innerHTML = 'x<sub>' + (i+1) + '</sub> &#8801; <b>' + xi[i] + '</b> (mod ' + modi[i] + ')'
        col.appendChild(row);
        solve_div.appendChild(col);
    }
    solve_div.appendChild(xi_table);
    // bnx column
    para = document.createElement('p')
    para.innerHTML = 'Step 4 - Multiply b<sub>i</sub>, N<sub>i</sub>, and x<sub>i</sub> together:'
    solve_div.appendChild(para);
    let bnx_table = document.createElement('table');
    bnx_table.classList.add('table');
    bnx_table.createTHead();
    bnx_table.tHead.innerHTML = '<tr><th>b<sub>i</sub></th><th>N<sub>i</sub></th><th>x<sub>i</sub></th><th>b<sub>i</sub>N<sub>i</sub>x<sub>i</sub></th></tr>'
    for(let i = 0; i < xi.length; i++) {
        let row = bnx_table.insertRow(i+1);
        row.insertCell(0).innerHTML = bi[i];
        row.insertCell(1).innerHTML = Ni[i];
        row.insertCell(2).innerHTML = xi[i];
        row.insertCell(3).innerHTML = bnx[i];
    }
    solve_div.appendChild(bnx_table);
    // Total column
    para = document.createElement('p')
    para.innerHTML = 'Step 5 - Add all elements of the b<sub>i</sub>N<sub>i</sub>x<sub>i</sub> column together:'
    solve_div.appendChild(para);
    bnx_table = document.createElement('table');
    bnx_table.classList.add('table');
    bnx_table.createTHead();
    bnx_table.tHead.innerHTML = '<tr><th>b<sub>i</sub></th><th>N<sub>i</sub></th><th>x<sub>i</sub></th><th>b<sub>i</sub>N<sub>i</sub>x<sub>i</sub></th></tr>'
    for(let i = 0; i < xi.length; i++) {
        let row = bnx_table.insertRow(i+1);
        row.insertCell(0).innerHTML = bi[i];
        row.insertCell(1).innerHTML = Ni[i];
        row.insertCell(2).innerHTML = xi[i];
        row.insertCell(3).innerHTML = bnx[i];
    }
    row = bnx_table.insertRow(modi.length + 1);
    row.insertCell(0).innerHTML;
    row.insertCell(1).innerHTML;
    row.insertCell(2).innerHTML = 'Total: ';
    row.insertCell(3).innerHTML = total;
    solve_div.appendChild(bnx_table);
    // Finish
    para = document.createElement('p')
    para.innerHTML = 'Step 6 - Mod total by N: '
    solve_div.appendChild(para);
    let end = document.createElement('table');
    end.classList.add('table');
    row = end.insertRow(0);
    let tmp = 'N = '
    for(let i = 0; i < Ni.length - 1; i++) {
        tmp += Ni[i] + ' * ';
    }
    tmp += Ni[Ni.length - 1] + '&rArr;';
    row.insertCell(0).innerHTML = tmp;

    row = end.insertRow(1);
    row.insertCell(0).innerHTML = 'x &#8801; ' + total + ' (mod ' + N + ') &rArr;'
    row = end.insertRow(2);
    row.insertCell(0).innerHTML = 'x &#8801; ' + answer + ' (mod ' + N + ')'
    solve_div.appendChild(end);
}
