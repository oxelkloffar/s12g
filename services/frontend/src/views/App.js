import React, { Component } from 'react'
import { Provider } from 'react-redux'
import AddPersonButton from '../components/AddPersonButton'
import PersonCount from '../components/PersonCount'
import './App.css'
import { createStore } from 'redux'
import { numberOfPeople } from '../reducers/numberOfPeople'


const store = createStore(numberOfPeople)



const App = () =>
  <Provider store={store}>
    <PersonCount />
    <AddPersonButton />
  </Provider>

export default App
