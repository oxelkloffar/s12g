import React, { Component } from 'react'
import { Provider } from 'react-redux'
import AddPerson from '../components/AddPerson'
import AncestryTree from '../components/AncestryTree/AncestryTree'
import './App.css'
import { createStore, combineReducers } from 'redux'
import persons from '../reducers/persons'
import selectedPerson from '../reducers/selectedPerson'
import InspectPerson from '../components/InspectPerson/InspectPerson'


const rootReducer = combineReducers({
  persons,
  selectedPerson
})
const store = createStore(rootReducer)



const App = () =>
  <Provider store={store}>
    <InspectPerson />
    <AncestryTree />
  </Provider>

export default App
