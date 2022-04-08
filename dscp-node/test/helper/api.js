const { ApiPromise, WsProvider } = require('@polkadot/api')
const { API_HOST, API_PORT } = require('../../src/env')
const logger = require('./logger')
const types = require('../../src/index')

const provider = new WsProvider(`ws://${API_HOST}:${API_PORT}`)
const apiOptions = {
  provider,
  types,
}

const api = new ApiPromise(apiOptions)

api.on('disconnected', () => {
  logger.warn(`Disconnected from substrate node at ${API_HOST}:${API_PORT}`)
})

api.on('connected', () => {
  logger.info(`Connected to substrate node at ${API_HOST}:${API_PORT}`)
})

api.on('error', (err) => {
  logger.error(`Error from substrate node connection. Error was ${err.message || JSON.stringify(err)}`)
})

module.exports = api