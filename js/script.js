document.addEventListener('DOMContentLoaded', function() {
    console.log('DOM fully loaded');
    // No need for the custom extension anymore
    
    const tableBody = document.getElementById('tableBody');
    let rowCounter = 0; // Start with 0 for the existing row
    
    // Add delete button to the first row
    const firstRow = tableBody.querySelector('tr');
    console.log('First row found:', firstRow);
    
    if (firstRow) {
        addDeleteButton(firstRow); // Make sure this executes
        console.log('Delete button added to first row');
    }
    
    // Function to add delete button to a row
    function addDeleteButton(row) {
        // Create a cell for the delete button
        const deleteCell = document.createElement('td');
        deleteCell.className = 'action-cell';
        
        // Create the delete button
        const deleteButton = document.createElement('button');
        deleteButton.innerHTML = '&times;';  // HTML entity for × (multiplication sign)
        deleteButton.className = 'delete-button';
        deleteButton.type = 'button';
        deleteButton.title = 'Delete row';
        deleteButton.style.backgroundColor = '#dc3545';
        deleteButton.style.color = 'white';
        deleteButton.style.border = 'none';
        deleteButton.style.width = '30px';
        deleteButton.style.height = '30px';
        deleteButton.style.fontSize = '20px';
        deleteButton.style.borderRadius = '4px';
        deleteButton.style.cursor = 'pointer';
        
        deleteButton.addEventListener('click', function() {
            // Don't delete if it's the only row
            console.log('Delete button clicked');
            if (tableBody.querySelectorAll('.data-row').length > 1) {
                row.remove();
                console.log('Row removed');
            } else {
                console.log('Cannot remove last row');
            }
        });
        
        deleteCell.appendChild(deleteButton);
        row.appendChild(deleteCell);
        console.log('Delete button cell added to row');
    }
    
    // Function to add a new row
    function addNewRow() {
        rowCounter++;
        const newRow = document.createElement('tr');
        newRow.className = 'data-row';
        newRow.innerHTML = `
            <td><input type="text" class="name-input" name="swimmers[${rowCounter}][name]" placeholder="Enter Name" /></td>
            <td><input type="number" class="age-input" name="swimmers[${rowCounter}][age]" placeholder="Enter Age" /></td>
            <td>
                <select class="skill-select" name="swimmers[${rowCounter}][skill]">
                    <option value="">Select Skill</option>
                    <option value="5">5</option>
                    <option value="6">6</option>
                    <option value="7">7</option>
                    <option value="8">8</option>
                    <option value="9">9</option>
                </select>
            </td>
            <td><input type="number" class="duration-input" name="swimmers[${rowCounter}][duration]" placeholder="Enter Duration" /></td>
        `;
        
        tableBody.appendChild(newRow);
        
        // Remove any validation attributes from the new inputs
        const newInputs = newRow.querySelectorAll('input, select');
        newInputs.forEach(input => {
            input.removeAttribute('required');
            input.setAttribute('novalidate', '');
        });
        
        // Add delete button to the new row
        addDeleteButton(newRow);
    }
    
    // Create the "Add Row" button
    const buttonContainer = document.querySelector('.button-container');
    console.log('Button container found:', buttonContainer);
    
    const addRowButton = document.createElement('button');
    addRowButton.textContent = 'добавить участника';
    addRowButton.className = 'add-row-button';
    addRowButton.type = 'button';
    addRowButton.addEventListener('click', function() {
        console.log('Add row button clicked');
        addNewRow();
    });
    
    // Insert the Add Row button before the submit button
    if (buttonContainer && buttonContainer.firstChild) {
        buttonContainer.insertBefore(addRowButton, buttonContainer.firstChild);
        console.log('Add row button inserted');
    } else if (buttonContainer) {
        buttonContainer.appendChild(addRowButton);
        console.log('Add row button appended');
    }

    // Make sure all inputs have required attribute removed
    const allInputs = document.querySelectorAll('input, select');
    allInputs.forEach(input => {
        input.removeAttribute('required');
    });

    // Add form submission handling
    const form = document.getElementById('swimmingForm');

    function cleanupEmptyRows() {
        const allRows = tableBody.querySelectorAll('.data-row');
        allRows.forEach(row => {
            const inputs = row.querySelectorAll('input, select');
            const allEmpty = Array.from(inputs).every(input => input.value.trim() === '');
            if (allEmpty) {
                row.remove();
            } else {
                // For non-empty rows, make sure all fields have values
                inputs.forEach(input => {
                    if (input.value.trim() === '') {
                        // Set a default value to prevent empty string parsing errors
                        if (input.type === 'number') {
                            input.value = '0';
                        } else if (input.tagName === 'SELECT') {
                            input.value = '5'; // Default to skill level 5
                        } else {
                            input.value = 'Unnamed';
                        }
                    }
                });
            }
        });
        
        // Make sure we have at least one row
        if (tableBody.children.length === 0) {
            addNewRow();
        }
    }

    // Function to reindex rows to ensure proper form submission
    function reindexRows() {
        const rows = tableBody.querySelectorAll('.data-row');
        rows.forEach((row, index) => {
            const inputs = row.querySelectorAll('input, select');
            inputs.forEach(input => {
                // Update the name attribute with new index
                const name = input.getAttribute('name');
                if (name) {
                    const newName = name.replace(/swimmers\[\d+\]/, `swimmers[${index}]`);
                    input.setAttribute('name', newName);
                }
            });
        });
    }

    form.addEventListener('submit', function(event) {
        event.preventDefault();
        
        cleanupEmptyRows();
        reindexRows();
        
        htmx.trigger(form, 'submit', {skipValidation: true});
    });
    
    document.body.addEventListener('htmx:configRequest', function(event) {
        if (event.detail.elt.id === 'swimmingForm') {
            cleanupEmptyRows();
            reindexRows();
            
            const params = event.detail.parameters;
            for (const key in params) {
                if (params[key] === '') {
                    delete params[key];
                }
            }
        }
    });
});