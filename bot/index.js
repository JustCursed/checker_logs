const { Client, Intents } = require('discord.js');
const client = new Client({ intents: [Intents.FLAGS.GUILDS] });
const fetch = require('node-fetch');
const split = require('lodash/split');
const reverse = require('lodash/reverse');
const config = require('./config.json');

const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

const request = async () => split(await (await fetch(`http://localhost:8080/shadowcraft/magic1710/index.html`)).text(), /\r\n|\r|\n/); // TODO: сделать красивее это уродство...

client.on('ready', async client => {
	console.log(`Logged in as ${client.user.tag}!`);

	const logs1 = await request();
	let lastLenght = logs1.length;
	setInterval(async () => {
		let logsArr = [];
		const logs2 = await request();

		for (let i = 0; i < logs2.length - lastLenght; i++) {
			let count = logs2.length - 2 - i;
			logsArr.push(logs2[count]);
			count++;
		}
		reverse(logsArr);
		lastLenght = logs2.length;
		console.log(logsArr);
		if (logsArr) {
			for (const log of logsArr) {
				client.channels.cache.get(config.chennalID).send(`\`${log}\``);
				await sleep(300);
			}
		}
	}, 1000);
});

client.login(config.token)/*.then(async () => {
	const logs1 = await request();
	let lastLenght = logs1.length;
	setInterval(async () => {
		let logsArr = [];
		const logs2 = await request();

		for (let i = 0; i < logs2.length - lastLenght; i++) {
			let count = logs2.length - 2 - i;
			logsArr.push(logs2[count]);
			count++;
		}
		reverse(logsArr);
		lastLenght = logs2.length;
		console.log(logsArr);
		if (logsArr) {
			for (const log of logsArr) {
				client.channels.cache.get(config.chennalID).send(`\`${log}\``);
				await sleep(300);
			}
		}
	}, 1000);
})*/;
