// webpack.config.js
const path = require('path');

module.exports = {
    mode: 'development',
    entry: {
        input: './scripts/input.js', // Ваш входной файл input.js
        bot: './scripts/bot.js'      // Ваш входной файл bot.js
    },
    output: {
        filename: '[name].bundle.js',
        path: path.resolve(__dirname, 'dist') 
    }
};
