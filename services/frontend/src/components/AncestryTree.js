import React from 'react'
import { connect } from 'react-redux'



const AncestryTreeComponent = ({ peeps }) =>
  <>
    Peeps: {peeps.map((peep, id) => (
      <p key={id}>{id} {peep}</p>
    ))}
  </>



const reduxStuff = connect(
  // fetch stuff from state into our props
  (state, ownProps) => ({
    peeps: state
  }),

  // functions which turn ui events into redux actions
  (dispatch, ownProps) => ({
  })
)

const AncestryTree = reduxStuff(AncestryTreeComponent)
export default AncestryTree
