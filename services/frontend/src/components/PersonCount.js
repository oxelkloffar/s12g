import React from 'react'
import { connect } from 'react-redux'



const PersonCountComponent = ({ peeps }) =>
  <p>
    Peeps: {peeps}
  </p>



const reduxStuff = connect(
  // fetch stuff from state into our props
  (state, ownProps) => ({
    peeps: state
  }),

  // functions which turn ui events into redux actions
  (dispatch, ownProps) => ({
  })
)

const PersonCount = reduxStuff(PersonCountComponent)
export default PersonCount
