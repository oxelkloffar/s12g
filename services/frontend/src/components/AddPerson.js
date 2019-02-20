import React from 'react'
import { connect } from 'react-redux'



const AddPersonComponent = ({ addPerson }) =>
  <div>
    <input type="text" placeholder="enter person name" />
    <button onClick={addPerson}>
      Add person
    </button>
  </div>



const reduxStuff = connect(
  // fetch stuff from state into our props
  (state, ownProps) => ({
    // right now we don't care about anything
    // from state in this component
  }),

  // functions which turn ui events into redux actions
  (dispatch, ownProps) => ({
    addPerson: (event) => {
      const inputElement = event.target.previousElementSibling
      console.log(inputElement.value)

      dispatch({
        type: 'ADD_PERSON',
        name: inputElement.value
      })
    }
  })
)

const AddPerson = reduxStuff(AddPersonComponent)
export default AddPerson
