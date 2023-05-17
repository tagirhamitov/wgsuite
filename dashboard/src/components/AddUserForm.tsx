import { Button, FormControl, FormLabel, TextField } from "@mui/material";
import React from "react";

function AddUserForm() {
  const [name, setName] = React.useState<string>("");

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setName(event.target.value);
  };

  return <FormControl sx={{ m: 5 }}>
    <TextField label="Name" onChange={handleChange}>{name}</TextField>
    <Button onClick={() => {
      fetch("http://localhost:3000/clients", {
        method: "post",
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ name: name })
      });
    }}>Create client</Button>
  </FormControl>;
}

export default AddUserForm;
