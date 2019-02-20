import React, { Component } from 'react'
import { Provider } from 'react-redux'
import AddPerson from '../components/AddPerson'
import AncestryTree from '../components/AncestryTree'
import './App.css'
import { createStore } from 'redux'
import { numberOfPeople } from '../reducers/numberOfPeople'


const store = createStore(numberOfPeople)



const App = () =>
  <Provider store={store}>
    <AncestryTree />
    <AddPerson />
  </Provider>

export default App
