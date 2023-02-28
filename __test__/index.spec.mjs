import test from 'ava'

import { initFaultHandler, triggerSignal } from '../index.js'

test('inits the handler', (t) => {
  initFaultHandler()

  t.pass()
})
