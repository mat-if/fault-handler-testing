import test from 'ava'

import { initFaultHandler, sum } from '../index.js'

test('inits the handler', (t) => {
  initFaultHandler()

  t.pass()
})
