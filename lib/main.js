const { Telegraf } = require('telegraf')
const engine = require('../')

const bot = new Telegraf(process.env.BOT_TOKEN)

bot.command('aposta', ctx => {
    engine.create_bet(ctx.message.text, ctx.message.chat.id)
    ctx.reply('Aposta feita com sucesso!')
})

bot.launch()