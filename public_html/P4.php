<?php

//Define the correct answers
$correct_answers=array(
	"a", "c", "a",
	"a", "c", "b",
	"a", "c", "a",
	"P4.php"
);

//Set score to 0;
$score=0;

//Loop through posted answers and compare with correct answers.
for($i=1;$i<=10;$i++)
{
	if(isset($_POST['q'.$i]))
	{
		if($_POST['q'.$i] == $correct_answers[$i-1])
		{
			$score++;
		}
	}
}

//Read first and last name.
$fname="";
$lname="";
if(isset($_POST['fname']))
{
	$fname = $_POST['fname'];
}
if(isset($_POST['lname']))
{
	$lname = $_POST['lname'];
}

//Set up credentials to connect to database.
$servername = "localhost";
$username = "cmps401";
$password = "Mycmps401db";
$db="cmps401";

//Connect
$mysqli = new mysqli($servername, $username, $password, $db);

//Check connection
if ($mysqli -> connect_errno) {
  echo "Failed to connect to MySQL: " . $mysqli -> connect_error;
  exit();
}


if($fname != "" && $lname != ""){
	//Check User
	$check_user = $mysqli -> prepare("SELECT * FROM g107 WHERE FirstName = ? AND LastName = ?");
	//Bind Data
	$check_user -> bind_param("ss", $fname, $lname);
	//Execute
	$check_user -> execute();
	//Count fields
	$check_user -> store_result();
	$field_cnt = $check_user->num_rows;
	//Free the data.
	$check_user -> free_result();
	
	//Process Results
	if ($field_cnt == 0){
		//Insert
		$insert = $mysqli -> prepare("INSERT INTO g107 (FirstName, LastName, Score, Time) VALUES (?, ?, ?, ?)");
		//Bind Data
		$time = date("Y-m-d H:m:s");
		$insert -> bind_param("ssss", $fname, $lname, $score, $time);
		//Execute
		$insert -> execute();
		$insert -> close();
	}else{
		//Update
		$update = $mysqli -> prepare("UPDATE g107 SET Score = ?, `Time` = ? WHERE FirstName = ? AND LastName = ?");
		//Bind Data
		$time = date("Y-m-d H:m:s");
		$update -> bind_param("ssss", $score, $time, $fname, $lname);
		//Execute
		$update -> execute();
		$update -> close();
	}
	
	$check_user -> close();
}else{
	$fname = "Unknown";
	$lname = "User";
}

$mysqli -> close();



echo"
<html>
<head>
	<title>Quiz Results: ".$fname." ".$lname." </title>
	<style>
		@import url('https://fonts.googleapis.com/css2?family=Open+Sans&display=swap');
		html{
			font-family:'Open Sans';
			background-color:#2A9F4B;
		}
		body{
			margin:0em;
			padding:0em;
			display:flex;
			height:100vh;
			width:100vw;
		}
		h2 {
			color:#2A9F4B;
		}
		h1{
			color:#2A9F4B;
		}
		div{
			border-radius:.1em;
			background-color:white;
			box-shadow:1px 1px 4px #888;
			margin:auto;
			padding:1em;
			text-align:center;
			width:20em;
		}
	</style>
</head>
<body>
	<div>
	<h2>".$fname." ".$lname." you scored:</h2>
	<h1>".$score."/10</h1>
	</div>
</body>
</html>
";
?>












