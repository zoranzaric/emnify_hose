// #![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate rand;

use std::env;
use std::io::{self, Write};

use rand::Rng;
use rand::ThreadRng;

use futures::Future;
use futures::stream::Stream;

use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, ContentType};

fn request(url: hyper::Uri, good: bool) -> Request {
    let json = if good {
        r#"{
  "id": 201370709,
  "alert": false,
  "description": "New location received from VLR for IMSI='90143012345678912345', now attached to VLR='491720013095'.",
  "timestamp": "2017-10-26T07:28:00.000+0000",
  "event_type": {
    "id": 11,
    "description": "Update location"
  },
  "event_source": {
    "id": 0,
    "description": "Network"
  },
  "event_severity": {
    "id": 0,
    "description": "Info"
  },
  "organisation": {
    "id": 839921,
    "name": "Demo Company"
  },
  "endpoint": {
    "id": 8638726,
    "name": "GPS Tracker",
    "ip_address": "100.96.234.249",
    "tags": null,
    "imei": "3577620833012201"
  },
  "imsi": {
    "id": 205672,
    "imsi": "90143012345678912345",
    "import_date": "2016-12-27T10:09:23.000+0000"
  },
  "sim": {
    "id": 274887,
    "iccid": "8988303001234567890",
    "production_date": "2016-12-27T10:09:23.000+0000"
  },
  "detail": {
    "id": 3,
    "name": "Vodafone",
    "country": {
      "id": 74,
      "name": "Germany",
      "country_code": "49",
      "mcc": "262",
      "iso_code": "de"
    },
    "tapcode": [{
      "id": 2,
      "tapcode": "DEUD2"
    }],
    "mnc": [{
      "id": 3,
      "mnc": "02"
    }]
  }
}"#
    } else {
        r#"{
  "id": 201370709,
  "alert": false,
  "description": "New location received from VLR for IMSI='90143012345678912345', now attached to VLR='491720013095'.",
  "timestamp": "2017-10-26T07:28:00.000+0000",
  "event_type": {
    "id": 1,
    "description": "Update location"
  },
  "event_source": {
    "id": 0,
    "description": "Network"
  },
  "event_severity": {
    "id": 0,
    "description": "Info"
  },
  "organisation": {
    "id": 839921,
    "name": "Demo Company"
  },
  "endpoint": {
    "id": 8638726,
    "name": "GPS Tracker",
    "ip_address": "100.96.234.249",
    "tags": null,
    "imei": "3577620833012201"
  },
  "imsi": {
    "id": 205672,
    "imsi": "90143012345678912345",
    "import_date": "2016-12-27T10:09:23.000+0000"
  },
  "sim": {
    "id": 274887,
    "iccid": "8988303001234567890",
    "production_date": "2016-12-27T10:09:23.000+0000"
  },
  "detail": {
    "id": 3,
    "name": "Vodafone",
    "country": {
      "id": 74,
      "name": "Germany",
      "country_code": "49",
      "mcc": "262",
      "iso_code": "de"
    },
    "tapcode": [{
      "id": 2,
      "tapcode": "DEUD2"
    }],
    "mnc": [{
      "id": 3,
      "mnc": "02"
    }]
  }
}"#
    };

    let mut request = Request::new(Method::Post, url);
    request.headers_mut().set(ContentType::json());
    request.headers_mut().set(ContentLength(json.len() as u64));
    request.set_body(json);

    request
}

fn rand(mut rng: ThreadRng) -> bool {
    let r = rng.gen::<u32>();

    r % 10000 == 0
}

fn main() {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure().build(&handle);

    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return;
        }
    };

    if url.parse::<hyper::Uri>().unwrap().scheme() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return;
    }



    let mut c = 0;
    let mut r = request(url.parse::<hyper::Uri>().unwrap(), true);

    loop {
        // let rng = rand::thread_rng();
        // request(rand(rng));
        if c % 180000 == 0 {
            r = request(url.parse::<hyper::Uri>().unwrap(), true);
        } else {
            r = request(url.parse::<hyper::Uri>().unwrap(), false);
        }

        let work = client.request(r);

        //    .and_then(|res| {
        //        println!("Response: {}", res.status());
        //        println!("Headers: \n{}", res.headers());
        //
        //        res.body().for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
        //    })
        //    .map(|_| {
        //        //            println!("\n\nDone.");
        //    });

        core.run(work).unwrap();

        c = c + 1;


        if c % 1000 == 0 {
            println!("{}", c);
        }
    }
}
