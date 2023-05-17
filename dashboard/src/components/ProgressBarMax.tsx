import React from 'react';
import { InputAdornment, TextField } from '@mui/material';

interface ProgressBarMaxProps {
  value: string;
  onValueChange: (value: string) => void;
}

function ProgressBarMax(props: ProgressBarMaxProps) {
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    props.onValueChange(event.target.value);
  }

  return <TextField
    id="outlined-number"
    label="Max data usage"
    type="number"
    value={props.value}
    onChange={handleChange}
    sx={{ mt: 5 }}
    InputProps={{
      startAdornment: <InputAdornment position="start">GB</InputAdornment>
    }}
    InputLabelProps={{
      shrink: true,
    }}
  />
}

export default ProgressBarMax;
