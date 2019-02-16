import React from 'react'
import { connect } from 'react-redux'



const AddPersonButtonComponent = ({ addPerson }) =>
  <button onClick={addPerson}>
    Add person
  </button>



const reduxStuff = connect(
  // fetch stuff from state into our props
  (state, ownProps) => ({
    // right now we don't care about anything
    // from state in this component
  }),

  // functions which turn ui events into redux actions
  (dispatch, ownProps) => ({
    addPerson: (event) => dispatch({ type: 'ADD_PERSON' })
  })
)

const AddPersonButton = reduxStuff(AddPersonButtonComponent)
export default AddPersonButton
