import React, { Component } from 'react'
import { Provider } from 'react-redux'
import './App.css'
import { createStore, combineReducers, applyMiddleware, compose } from 'redux'
import persons from '../reducers/persons/persons'
import selectedPerson from '../reducers/selectedPerson'
import addPerson from '../reducers/addPerson'
import AddPerson from '../components/AddPerson/AddPerson'
import AncestryTree from '../components/AncestryTree/AncestryTree'
import InspectPerson from '../components/InspectPerson/InspectPerson'


const rootReducer = combineReducers({
  persons,
  selectedPerson,
  addPerson
})
const composeEnhancers = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose
const store = createStore(
  rootReducer,
  composeEnhancers(
    applyMiddleware(store => next => action => {
      console.group("Action:", action.type)
      console.info('dispatching', action)
      const result = next(action)
      console.log('next state', store.getState())
      console.groupEnd()
      return result
    })
  )
)



const App = () =>
  <Provider store={store}>
    <AddPerson />
    <InspectPerson />
    <AncestryTree />
  </Provider>

export default App
