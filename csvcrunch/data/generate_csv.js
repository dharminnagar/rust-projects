const fs = require('fs');

let csvContent = "id,name,value\n";
for (let i = 1; i <= 50; i++) {
    csvContent += `${i},Name_${i},${(Math.random() * 100).toFixed(2)}\n`;
}

fs.writeFileSync('test_data.csv', csvContent);
console.log("Successfully generated 50 lines of CSV in test_data.csv");
